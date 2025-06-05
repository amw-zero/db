# $DB

$DB is a relational database implementation, 

# Design Goals

* Simplicity
* Edification
* Configurability
* Extensibility
* Observability
* Simulatability

## Simplicity

## Edification

# Postgres Reference Links

While not a Postgres clone, $DB bases much of its design on Postgres.  Here are relevant links to the PG source:

[Heap access methods](https://github.com/postgres/postgres/blob/master/src/include/access/heapam.h#L288). The Heap is Postgres' abstraction around persisted data.

[PG file layout](https://www.postgresql.org/docs/current/storage-file-layout.html)

[PG Page Definition](https://github.com/postgres/postgres/blob/aa87f69c009a062685f0c984dbcc18e60c02920d/src/include/storage/bufpage.h#L25-L82)

[PG Buffer Manager Definition](https://github.com/postgres/postgres/blob/73bdcfab35ec0a7eff1a5dd630cbad8e77054547/src/include/storage/bufmgr.h)

[Read Buffer](https://github.com/postgres/postgres/blob/master/src/include/storage/bufmgr.h#L211)

# Testing Links

[MongoDB Spec-based testing](https://www.mongodb.com/blog/post/engineering/conformance-checking-at-mongodb-testing-our-code-matches-our-tla-specs)
