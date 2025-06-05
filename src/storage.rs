use crate::types::Tuple;

pub trait Storage {
    fn insert(&mut self, tup: Tuple, table: &str);
    fn select(&self, table: &str) -> Vec<Tuple>;
}
