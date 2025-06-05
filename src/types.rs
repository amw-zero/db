#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
}

pub type Tuple = Vec<Value>;
