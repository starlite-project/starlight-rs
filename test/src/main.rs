#![feature(backtrace)]

use star_lang::*;

fn main() {
    let lang = LangMap::from_dir("./languages").unwrap();

    let english = lang.get("en_us").unwrap();

    dbg!(english.clone());

    let ping = english.get("ping").unwrap();

    dbg!(ping.clone());

    dbg!(ping.write().unwrap());

    let pong = english.get("pong").unwrap();

    dbg!(pong.clone());

    dbg!(pong.write_params(&["10"]).unwrap());
}
