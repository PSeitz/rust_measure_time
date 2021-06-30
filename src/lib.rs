//! The crate provides macros, which measure the time in ms until end of scope
//!
//! This is done by creating an object, which measures the time. The time is printed when the object is dropped.
//!
//! The logging behaviour is the same as other log macros like info!(..)
//!
//! ### Examples
//!
//! ```rust
//! use measure_time::{info_time, debug_time, trace_time, error_time, print_time};
//! fn main() {
//!     info_time!("measure function");
//!     {
//!         debug_time!("{:?}", "measuring block");
//!         let mut sum = 0;
//!         for el in 0..50000 {
//!             sum+=el;
//!         }
//!         println!("{:?}", sum);
//!     }
//!     trace_time!("{:?}", "trace");
//!     print_time!("print");
//!     error_time!(target: "measure_time", "custom target");
//!     // --> prints "yep took ...", "measuring function took 0.0135ms"
//! }
//! ```
//!
extern crate log;
pub use log::*;

#[macro_export]
macro_rules! log_time {
    (target: $target:expr, $lvl:expr, $lvl2:expr, $($arg:tt)+) => (
        #[allow(unused_variables)]
        let time = if $crate::log_enabled!($lvl) {
            Some($crate::MeasureTime::new($target, module_path!(), file!(), line!(), format!($($arg)+), $lvl2) )
        } else{
            None
        };
    );
}

/// logs the time with the error! macro
#[macro_export]
macro_rules! error_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Error, $crate::LevelFilter::Error, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Error, $crate::LevelFilter::Error, $($arg)+) )
}
/// logs the time with the warn! macro
#[macro_export]
macro_rules! warn_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Warn, $crate::LevelFilter::Warn, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Warn, $crate::LevelFilter::Warn, $($arg)+) )
}
/// logs the time with the info! macro
#[macro_export]
macro_rules! info_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Info, $crate::LevelFilter::Info, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Info, $crate::LevelFilter::Info, $($arg)+) )
}
/// logs the time with the debug! macro
#[macro_export]
macro_rules! debug_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Debug, $crate::LevelFilter::Debug, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Debug, $crate::LevelFilter::Debug, $($arg)+) )
}
/// logs the time with the trace! macro
#[macro_export]
macro_rules! trace_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Trace, $crate::LevelFilter::Trace, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Trace, $crate::LevelFilter::Trace, $($arg)+) )
}
/// logs the time with the print! macro
#[macro_export]
macro_rules! print_time {($($arg:tt)+) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new(module_path!(), module_path!(), file!(), line!(), format!($($arg)+), $crate::LevelFilter::Off); } }

#[cfg(test)]
mod tests {
    #[test]
    fn funcy_func() {
        trace_time!("{:?}", "trace");
        debug_time!("{:?}", "debug");
        info_time!("measure function");
        {
            debug_time!("{:?}", "measuring block");
            let mut sum = 0;
            for el in 0..50000 {
                sum += el;
            }
            println!("{:?}", sum);
        }

        print_time!("print");
        error_time!("error_time");

        trace_time!(target: "trace_time", "custom target");
        debug_time!(target: "debug_time", "custom target");
        info_time!(target: "info_time", "custom target");
        error_time!(target: "measure_time", "custom target");
    }
}

#[derive(Debug)]
pub struct MeasureTime {
    name: String,
    target: &'static str,
    module_path: &'static str,
    file: &'static str,
    line: u32,
    start: std::time::Instant,
    level: log::LevelFilter,
}
impl MeasureTime {
    pub fn new<S: Into<String>>(
        target: &'static str,
        module_path: &'static str,
        file: &'static str,
        line: u32,
        name: S,
        level: log::LevelFilter,
    ) -> Self {
        MeasureTime {
            target,
            module_path,
            file,
            line,
            name: name.into(),
            start: std::time::Instant::now(),
            level,
        }
    }
}

impl Drop for MeasureTime {
    fn drop(&mut self) {
        let time_in_ms = (self.start.elapsed().as_secs() as f64 * 1_000.0)
            + (self.start.elapsed().subsec_nanos() as f64 / 1000_000.0);

        let time = match time_in_ms as u64 {
            0..=3000 => format!("{}ms", time_in_ms),
            3001..=60000 => format!("{:.2}s", time_in_ms / 1000.0),
            _ => format!("{:.2}m", time_in_ms / 1000.0 / 60.0),
        };

        if let Some(level) = self.level.to_level() {
            log::logger().log(
                &log::Record::builder()
                    .args(format_args!("{} took {}", self.name, time))
                    .level(level)
                    .target(self.target)
                    .module_path(Some(self.module_path))
                    .file(Some(self.file))
                    .line(Some(self.line))
                    .build(),
            );
        } else {
            println!("{} took {}", self.name, time);
        }
    }
}
