use bincode::serialize;
use serde::{Deserialize, Serialize};
use serde_cbor::to_vec;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u64,
    height: u64,
    weight: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let person = Person {
        name: "Owen".to_owned(),
        age: 21,
        height: 70,
        weight: 138
    };

    dbg!(to_vec(&person).unwrap().len());

    dbg!(serialize(&person).unwrap().len());

    Ok(())
}
