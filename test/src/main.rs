use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn main() {
    println!("Waiting until being aborted");

    loop {
        thread::sleep(Duration::from_millis(200));
        print!(".");
        stdout().flush().ok();
    }
}
