# $DB

$DB is a relational database implementation, focused on:

* Simplicity
* Edification
* Configurability
* Extensibility
* Observability
* Testability

The goal of $DB is not to create an industrial-grade database. Rather, it's to create a simple engine that is both configurable and extensible so that we can learn through experimentation.

We also want to focus on internal observability, to see what design choices can be made to better enable the observation of database functionality and performance.

Additionally, testability is a first-class goal, where we want to use approaches such as deterministic simulation testing to focus on defining and checking for holistic correctness properties.

# Running

For now, $DB is run via tests. Run `cargo test` to run the whole test suite. To add a new test, use the `mk_db` test helper to create a `DB` struct, and use the DB interface functions to manipulate its data:

```
use db::db::{insert, select, DB};
use db::buffer_mgr::BufferMgr;
use db::types::Value;

// Choose a storage implementation, this one being the
// BufferMgr which manages in-memory buffers of block data.
let buffer_mgr = BufferMgr::new();
let mut db = tests::mk_db(buffer_mgr);

let tup = vec![Value::Int(1)];
insert(tup, "tbl".to_string(), &mut db);

let selected = select("tbl", &db);

assert_eq!(selected, vec![vec![Value::Int(1)]]);
```

See more tests in [tests/test.rs](https://github.com/amw-zero/db/blob/main/tests/test.rs).

# Postgres Reference Links

While not a Postgres clone, $DB bases much of its design on Postgres. Here are relevant links to the PG source:

[Heap access methods](https://github.com/postgres/postgres/blob/master/src/include/access/heapam.h#L288). The Heap is Postgres' abstraction around persisted data.

[PG file layout](https://www.postgresql.org/docs/current/storage-file-layout.html)

[PG Page Definition](https://github.com/postgres/postgres/blob/aa87f69c009a062685f0c984dbcc18e60c02920d/src/include/storage/bufpage.h#L25-L82)

[PG Buffer Manager Definition](https://github.com/postgres/postgres/blob/73bdcfab35ec0a7eff1a5dd630cbad8e77054547/src/include/storage/bufmgr.h)

[Read Buffer](https://github.com/postgres/postgres/blob/master/src/include/storage/bufmgr.h#L211)

# Testing Links

[MongoDB Spec-based testing](https://www.mongodb.com/blog/post/engineering/conformance-checking-at-mongodb-testing-our-code-matches-our-tla-specs)
