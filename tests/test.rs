use reldb::db::{DB, insert, select};
use reldb::types::{Value, Tuple};
use reldb::buffer_mgr::BufferMgr;
use reldb::mem_storage::MemStorage;
use arbtest::arbtest;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub enum Action {
        Insert(Vec<Value>),
        Select(String),
    }

    pub fn mk_db() -> DB<BufferMgr> {
        let buffer_mgr: BufferMgr = BufferMgr::new();
        DB {
            name: "test".to_string(),
            storage: buffer_mgr,
        }
    }

    pub fn run_actions(actions: Vec<Action>, db: &mut DB<BufferMgr>) {
        for action in actions {
            match action {
                Action::Insert(values) => {
                    let tup = values;
                    insert(tup, "tbl".to_string(), db);
                }
                Action::Select(table) => {
                    let _ = select(&table, db);
                }
            }
        }
    }
}

// create table test (val int);
// insert into test values (1);
// select * from table;
#[test]
fn test_insert_and_select() {
//    let buffer_mgr = BufferMgr::new();
    let mem = MemStorage::new();
    let mut db = DB {
        name: "test".to_string(),
        storage: mem,
    };

//    let tup = vec![Value::Int(1)];
//    insert(tup, "tbl".to_string(), &mut db);

//    let selected = select("test", &db);
//    assert_eq!(selected.len(), 1);
//    assert_eq!(selected[0][0], Value::Int(1));
}

#[test]
fn test_actions() {
    arbtest(|u| {
        let mut db = tests::mk_db();
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
            } else if act == 1{
                // Select action
                actions.push(tests::Action::Select("tbl".to_string()));
            }
        }

        println!("Generated actions: {:?}", actions);

        for action in &actions {
            match action {
                tests::Action::Insert(values) => {
                    let tup = values.clone();
                    insert(tup, "tbl".to_string(), &mut db);

                    // Check insert invariants
                }
                tests::Action::Select(table) => {
                    let _ = select(table, &db);

                    // Check select invariants
                }
            }

            // Check global invariants
        }

        Ok(())
    });
}


