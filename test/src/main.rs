use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Data {
    Integer(u64),
    Pair(String, String)
}

fn main() {
    let data = Data::Integer(10);

    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    let data = "10";

    dbg!(serde_json::from_str::<Data>(&data).unwrap());
}
