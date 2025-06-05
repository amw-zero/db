#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
}

pub type Tuple = Vec<Value>;
