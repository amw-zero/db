use crate::types::Tuple;

pub struct Block {

}

// create table test (val int);
// insert into test values (1);
// select * from table;
pub struct DB {
    pub name: String,
    pub blocks: Vec<Block>,
}

pub fn insert(tup: Tuple, table: String, db: &mut DB) {
    let block = tuple_to_block(tup);
    db.blocks.push(block);
}

fn block_to_tuple(block: &Block) -> Tuple {
    // Convert a Block to a Tuple
    vec![] // Placeholder, actual conversion logic needed
}
fn tuple_to_block(tup: Tuple) -> Block {
    // Convert a Tuple to a Block
    Block {} // Placeholder, actual conversion logic needed
}

pub fn select(table: &str, db: &DB) -> Vec<Tuple>{
    let mut result = Vec::new();
    for block in &db.blocks {
        let tup = block_to_tuple(block);
        result.push(tup);
    }

    result

}