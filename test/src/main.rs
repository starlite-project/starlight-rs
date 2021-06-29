#![feature(backtrace)]

use star_lang::*;

fn main() {
    let lang = LangMap::from_dir("./languages").unwrap();

    dbg!(lang.get("en_us").unwrap());
}
