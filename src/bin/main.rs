use reldb::types::{Value};
use reldb::db::{DB, insert, select};
use reldb::buffer_mgr::BufferMgr;


fn main() {
    let buffer_mgr: BufferMgr = BufferMgr::new();
    let mut db = DB {
        name: "test".to_string(),
        storage: buffer_mgr,
    };

    let tup = vec![Value::Int(1)];
    insert(tup, "tbl".to_string(), &mut db);

    select("tbl", &db);
}
