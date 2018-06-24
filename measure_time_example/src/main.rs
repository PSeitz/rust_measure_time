extern crate env_logger;
#[macro_use]
extern crate measure_time;

fn main() {
    env_logger::init();
    error_time!("error_time");

    error_time!(target: "measure_time_examplero","error_time custom target");

    use std::{thread, time};

    let one_sec = time::Duration::from_millis(1000);
    thread::sleep(one_sec);

}

