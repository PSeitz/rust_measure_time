max_width_relation 1.0
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate measure_time;
#[macro_use]
extern crate std;

fn main() {
    env_logger::init();
    let time = 10;

    #[allow(unused_variables)]
    let time = if {
        let lvl = ::log::Level::Error;
        lvl <= ::STATIC_MAX_LEVEL && lvl <= ::max_level()
            && ::__private_api_enabled(lvl, "measure_time_example")
    } {
        Some(::MeasureTime::new(
            "measure_time_example",
            "measure_time_example",
            "src/main.rs",
            10u32,
            ::fmt::format(::std::fmt::Arguments::new_v1(
                &["error_time"],
                &match () {
                    () => [],
                },
            )),
            ::log::LevelFilter::Error,
        ))
    } else {
        None
    };
    #[allow(unused_variables)]
    let time = if {
        let lvl = ::log::Level::Error;
        lvl <= ::STATIC_MAX_LEVEL && lvl <= ::max_level()
            && ::__private_api_enabled(lvl, "measure_time_example")
    } {
        Some(::MeasureTime::new(
            "measure_time_example",
            "measure_time_example",
            "src/main.rs",
            13u32,
            ::fmt::format(::std::fmt::Arguments::new_v1(
                &["test"],
                &match () {
                    () => [],
                },
            )),
            ::log::LevelFilter::Error,
        ))
    } else {
        None
    };
    use std::{thread, time};
    ::io::_print(::std::fmt::Arguments::new_v1_formatted(
        &["", "\n"],
        &match (&time,) {
            (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
        },
        &[
            ::std::fmt::rt::v1::Argument {
                position: ::std::fmt::rt::v1::Position::At(0usize),
                format: ::std::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                    flags: 0u32,
                    precision: ::std::fmt::rt::v1::Count::Implied,
                    width: ::std::fmt::rt::v1::Count::Implied,
                },
            },
        ],
    ));
    let one_sec = time::Duration::from_millis(1000);
    thread::sleep(one_sec);
}
