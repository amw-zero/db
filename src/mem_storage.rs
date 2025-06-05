use std::collections::HashMap;
use crate::types::Tuple;
use crate::storage::Storage;
pub struct MemStorage {
    data: HashMap<String, Vec<Tuple>>,
}

impl MemStorage {
    pub fn new() -> Self {
        MemStorage {
            data: HashMap::new(),
        }
    }
}

impl Storage for MemStorage {
    fn insert(&mut self, tup: Tuple, table: &str) {
        self.data.entry(table.to_string())
            .or_insert_with(Vec::new)
            .push(tup);
    }

    fn select(&self, table: &str) -> Vec<Tuple> {
        self.data.get(table).unwrap_or(&vec![vec![]]).clone()
    }
}