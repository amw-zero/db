use arbtest::arbtest;
use db::buffer_mgr::BufferMgr;
use db::db::{insert, select};
use db::types::Value;

mod test_util;

#[test]
fn test_insert_and_select() {
    let buffer_mgr = BufferMgr::new();
    let mut db = test_util::mk_db(buffer_mgr);

    let tup = vec![Value::Int(1)];
    insert(tup, "tbl".to_string(), &mut db);

    let selected = select("tbl", &db);

    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0][0], Value::Int(1));
}

#[test]
fn test_multi_insert() {
    let buffer_mgr = BufferMgr::new();
    let mut db = test_util::mk_db(buffer_mgr);

    let tup = vec![Value::Int(1)];
    insert(tup, "tbl".to_string(), &mut db);
    let tup2 = vec![Value::Int(2)];
    insert(tup2, "tbl".to_string(), &mut db);

    let selected = select("tbl", &db);

    assert_eq!(selected, vec![vec![Value::Int(1)], vec![Value::Int(2)]]);
}

// Failing seed: 0xfb7b652500000020
#[test]
fn test_actions() {
    arbtest(test_util::property).budget_ms(5_000).run();
}
