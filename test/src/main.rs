use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct World(Vec<Entity>);

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./file.cbor")?;

    let deserialized: World = serde_cbor::from_reader(&file)?;

    dbg!(deserialized);

    Ok(())
}
