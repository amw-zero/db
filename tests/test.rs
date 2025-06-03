use reldb::db::{DB, insert, select};
use reldb::types::{Value};

#[test]
fn test_insert_and_select() {
    let mut db = DB {
        name: "test".to_string(),
        blocks: vec![],
    };

    let tup = vec![Value::Int(1)];
    insert(tup, "tbl".to_string(), &mut db);

    let selected = select("test", &db);
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0][0], Value::Int(1));
}
