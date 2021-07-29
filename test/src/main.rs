use self_meter::Meter;
use star_test::Grid;
use std::{
    collections::BTreeMap,
    io::{stderr, Write},
    time::Duration,
    thread::sleep
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut meter = Meter::new(Duration::new(1, 0))?;
    meter.track_current_thread("main");
    loop {
        meter
            .scan()
            .map_err(|e| writeln!(&mut stderr(), "Scan error: {}", e))
            .ok();

        println!("Report: {:?}", meter.report());
        println!(
            "Threads: {:?}",
            meter.thread_report().map(|x| x.collect::<BTreeMap<_, _>>())
        );

        let val = Box::new(10);

        sleep(Duration::new(1, 0));
    }
}
