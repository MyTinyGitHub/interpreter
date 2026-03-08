//! Environment for variable bindings and closures.
//!
//! Maps variable names to values and implements lexical scoping.
//! The `outer` field enables closures by capturing the defining scope.
//!
//! # How Closures Work
//!
//! 1. When a function is defined, it captures the current environment
//! 2. When called, a new environment is created that encloses the captured one
//! 3. Variable lookup checks local scope first, then chains outward
//!
//! This ensures functions "remember" where they were defined, not where they're called.

use std::collections::HashMap;

use crate::evaluator::object::{Function, Object};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_enclosed(env: &Environment) -> Self {
        Self {
            store: HashMap::default(),
            outer: Some(Box::new(env.clone())),
        }
    }

    pub fn get(&self, value: &str) -> Option<&Object> {
        self.store
            .get(value)
            .or_else(|| self.outer.as_ref()?.get(value))
    }

    pub fn set(&mut self, value: &str, object: Object) {
        self.store.insert(value.to_string(), object);
    }

    pub fn extend_func_env(&self, func: &Function, args: Vec<Object>) -> Environment {
        let mut extended = Environment::new_enclosed(self);

        for (parameter, arg) in func.parameters.iter().zip(args) {
            extended.set(parameter.token.literal(), arg);
        }

        extended
    }
}
