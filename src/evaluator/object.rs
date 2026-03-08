use crate::{
    ast::{BlockStatement, Identifier},
    evaluator::environment::Environment,
};

type ObjectType = String;

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Return(Box<Object>),
    Function(Function),
    Null,
}

impl Object {
    pub fn obj_type(&self) -> ObjectType {
        match self {
            Self::Integer(_) => "INTEGER".to_string(),
            Self::Boolean(_) => "BOOLEAN".to_string(),
            Self::Return(_) => "RETURN_VALUE".to_string(),
            Self::Function(_) => "FUNCTION".to_string(),
            Self::Null => "NULL".to_string(),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Self::Integer(val) => val.to_string(),
            Self::Boolean(val) => val.to_string(),
            Self::Return(val) => val.inspect(),
            Self::Function(val) => val.inspect(),
            Self::Null => "null".to_string(),
        }
    }
}

impl Function {
    pub fn inspect(&self) -> String {
        format!(
            "fn({}){{\n{}\n}}",
            self.parameters
                .iter()
                .map(|v| v.token.literal())
                .collect::<Vec<_>>()
                .join(","),
            self.body.string()
        )
    }
}
