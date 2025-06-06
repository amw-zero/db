use db::buffer_mgr::BufferMgr;
use db::db::{insert, select, DB};
use db::mem_storage::MemStorage;
use db::storage::Storage;
use db::types::Value;

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

pub fn property(u: &mut arbitrary::Unstructured) -> arbitrary::Result<()> {
    let buffer_mgr = BufferMgr::new();
    let mut db = mk_db(buffer_mgr);

    let mem = MemStorage::new();
    let mut model = mk_db(mem);

    let num_actions = u.int_in_range(0..=100)?;
    let mut actions: Vec<Action> = Vec::new();

    for i in 0..num_actions {
        let act = u.int_in_range(0..=1)?;
        if act == 0 {
            let values: Vec<Value> = (0..u.int_in_range(1..=5)?)
                .map(|_| u.int_in_range(0..=100))
                .filter(|v| v.is_ok())
                .map(|v| Value::Int(v.unwrap()))
                .collect();
            actions.push(Action::Insert(values));
        } else if act == 1 {
            // Select action
            actions.push(Action::Select("tbl".to_string()));
        }
    }

    for action in &actions {
        println!("Executing action: {:?}", action);
        match action {
            Action::Insert(values) => {
                let tup = values.clone();

                insert(tup.clone(), "tbl".to_string(), &mut db);
                insert(tup, "tbl".to_string(), &mut model);
            }
            Action::Select(table) => {
                let db_result = select(table, &db);
                let model_result = select(table, &model);

                if db_result != model_result {
                    println!("Mismatch in select results: {:?}", db.storage);
                }

                assert_eq!(
                    db_result, model_result,
                    "Mismatch in select results for table: {}",
                    table
                );
            }
        }
    }

    Ok(())
}
