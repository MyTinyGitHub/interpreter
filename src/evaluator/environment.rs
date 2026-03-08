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

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::evaluator::object::{Function, Object};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new_enclosed(env: Rc<RefCell<Environment>>) -> Self {
        Self {
            store: HashMap::default(),
            outer: Some(env),
        }
    }

    pub fn get(&self, value: &str) -> Option<Object> {
        self.store
            .get(value)
            .cloned()
            .or_else(|| self.outer.as_ref()?.borrow().get(value))
    }

    pub fn set(&mut self, value: &str, object: Object) {
        self.store.insert(value.to_string(), object);
    }
}

pub fn extend_func_env(func: &Function, args: Vec<Object>) -> Rc<RefCell<Environment>> {
    let mut extended = Environment::new_enclosed(Rc::clone(&func.env));

    for (parameter, arg) in func.parameters.iter().zip(args) {
        extended.set(parameter.token.literal(), arg);
    }

    Rc::new(RefCell::new(extended))
}
