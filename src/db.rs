use crate::storage::Storage;
use crate::types::Tuple;

pub struct DB<S: Storage> {
    pub name: String,
    pub storage: S,
}

pub fn insert<S: Storage>(tup: Tuple, table: String, db: &mut DB<S>) {
    db.storage.insert(tup, &table);
}

pub fn select<S: Storage>(table: &str, db: &DB<S>) -> Vec<Tuple> {
    db.storage.select(table)
}
