struct DBModel {
    name: String,
    tables: Map<String, Vec<Tuple>>,
}

fn ModelInsert(db: &DBModel, table: &str, tup: Tuple) {
    if let Some(tuples) = db.tables.get_mut(table) {
        tuples.push(tup);
    } else {
        db.tables.insert(table.to_string(), vec![tup]);
    }
}

fn ModelSelect<'a>(db: &'a DBModel, table: &str) -> Option<&'a Vec<Tuple>> {
    db.tables.get(table)
}