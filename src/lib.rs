//! The crate provides macros, which measure the time in ms until end of scope
//!
//! This is done by creating an object, which measures the time. The time is printed when the object is dropped.
//!
//! *log verson = "0.4"  is required*
//!
//! Currently macro re-export is not supported in rust, so the user needs to ```macro_use``` import the log module like in the example below:
//!
//!
//! ### Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate measure_time;
//! fn main() {
//!     info_time!("measure function");
//!     {
//!         debug_time!(format!("{:?}", "measuring block"));
//!         let mut sum = 0;
//!         for el in 0..50000 {
//!             sum+=el;
//!         }
//!         println!("{:?}", sum);
//!         // --> prints "measuring block took 0.010ms"
//!     }
//!     trace_time!(format!("{:?}", "yep"));
//!     // --> prints "yep took ...", "measuring function took 0.0135ms"
//! }
//! ```
//!


#[macro_use]
pub extern crate log;

/// logs the time with the error! macro
#[macro_export]
macro_rules! error_time {($e:expr) => {#[allow(unused_variables)] let time = if log_enabled!($crate::log::Level::Error) { Some($crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Error) ) } else{ None }; } }
/// logs the time with the warn! macro
#[macro_export]
macro_rules! warn_time {($e:expr) =>  {#[allow(unused_variables)] let time = if log_enabled!($crate::log::Level::Warn) { Some($crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Warn) ) } else{ None }; } }
/// logs the time with the info! macro
#[macro_export]
macro_rules! info_time {($e:expr) =>  {#[allow(unused_variables)] let time = if log_enabled!($crate::log::Level::Info) { Some($crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Info) ) } else{ None }; } }
/// logs the time with the debug! macro
#[macro_export]
macro_rules! debug_time {($e:expr) => {#[allow(unused_variables)] let time = if log_enabled!($crate::log::Level::Debug) { Some($crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Debug) ) }else{ None }; } }
/// logs the time with the trace! macro
#[macro_export]
macro_rules! trace_time {($e:expr) => {#[allow(unused_variables)] let time = if log_enabled!($crate::log::Level::Trace) { Some($crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Trace) ) } else{ None }; } }
/// logs the time with the print! macro
#[macro_export]
macro_rules! print_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Print); } }

#[derive(Debug)]
pub enum MeasureTimeLogLevel {Error, Warn, Info, Debug, Trace, Print}

#[derive(Debug)]
pub struct MeasureTime {
    name: String,
    start: std::time::Instant,
    level: MeasureTimeLogLevel
}
impl MeasureTime {
    pub fn new<S: Into<String>>(name: S, level:MeasureTimeLogLevel) -> Self {MeasureTime{name:name.into(), start: std::time::Instant::now(), level:level} }
}

impl Drop for MeasureTime {
    fn drop(&mut self) {
        let time_in_ms = (self.start.elapsed().as_secs() as f64 * 1_000.0) + (self.start.elapsed().subsec_nanos() as f64 / 1000_000.0);

        let time = match time_in_ms as u64 {
            0..=3000 => format!("{}ms", time_in_ms),
            3000..=60000 => format!("{:.2}s", time_in_ms/1000.0),
            _ => format!("{:.2}m", time_in_ms/1000.0/60.0),
        };
        match self.level  {
            MeasureTimeLogLevel::Error =>   error!("{} took {}",self.name, time),
            MeasureTimeLogLevel::Warn  =>    warn!("{} took {}",self.name, time),
            MeasureTimeLogLevel::Info  =>    info!("{} took {}",self.name, time),
            MeasureTimeLogLevel::Debug =>   debug!("{} took {}",self.name, time),
            MeasureTimeLogLevel::Trace =>   trace!("{} took {}",self.name, time),
            MeasureTimeLogLevel::Print => println!("{} took {}",self.name, time),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn funcy_func() {
        info_time!("measure function");
        {
            debug_time!(format!("{:?}", "measuring block"));
            let mut sum = 0;
            for el in 0..50000 {
                sum+=el;
            }
            println!("{:?}", sum);
        }
        trace_time!(format!("{:?}", "yep"));
        print_time!("yep2");
    }
}
