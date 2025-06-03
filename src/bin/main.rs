use reldb::types::{Tuple, Value};
use reldb::db::{DB, insert, select};


fn main() {
    let mut db = DB {
        name: "test".to_string(),
        blocks: vec![],
    };

    let tup = vec![Value::Int(1)];
    insert(tup, "tbl".to_string(), &mut db);

    select("tbl", &db);
}
