use arbtest::arbtest;
use db::buffer_mgr::BufferMgr;
use db::db::{insert, select, DB};
use db::mem_storage::MemStorage;
use db::storage::Storage;
use db::types::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub enum Action {
        Insert(Vec<Value>),
        Select(String),
    }

    pub fn mk_db<S: Storage>(storage: S) -> DB<S> {
        DB {
            name: "test".to_string(),
            storage: storage,
        }
    }
}

#[test]
fn test_insert_and_select() {
    let buffer_mgr = BufferMgr::new();
    let mut db = tests::mk_db(buffer_mgr);

    let tup = vec![Value::Int(1)];
    insert(tup, "tbl".to_string(), &mut db);

    let selected = select("tbl", &db);

    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0][0], Value::Int(1));
}

#[test]
fn test_multi_insert() {
    let buffer_mgr = BufferMgr::new();
    let mut db = tests::mk_db(buffer_mgr);

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
    arbtest(|u| {
        let buffer_mgr = BufferMgr::new();
        let mut db = tests::mk_db(buffer_mgr);

        let mem = MemStorage::new();
        let mut model = tests::mk_db(mem);

        let num_actions = u.int_in_range(0..=100)?;
        let mut actions: Vec<tests::Action> = Vec::new();

        for i in 0..num_actions {
            let act = u.int_in_range(0..=1)?;
            if act == 0 {
                let values: Vec<Value> = (0..u.int_in_range(1..=5)?)
                    .map(|_| u.int_in_range(0..=100))
                    .filter(|v| v.is_ok())
                    .map(|v| Value::Int(v.unwrap()))
                    .collect();
                actions.push(tests::Action::Insert(values));
            } else if act == 1 {
                // Select action
                actions.push(tests::Action::Select("tbl".to_string()));
            }
        }

        for action in &actions {
            match action {
                tests::Action::Insert(values) => {
                    let tup = values.clone();

                    insert(tup.clone(), "tbl".to_string(), &mut db);
                    insert(tup, "tbl".to_string(), &mut model);
                }
                tests::Action::Select(table) => {
                    let db_result = select(table, &db);
                    let model_result = select(table, &model);

                    assert_eq!(
                        db_result, model_result,
                        "Mismatch in select results for table: {}",
                        table
                    );
                }
            }
        }

        Ok(())
    })
    .budget_ms(5_000)
    .run();
}
