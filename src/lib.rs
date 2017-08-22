//! The crate provides macros, which measure the time in ms until end of scope
//!
//! This is done by creating an object, which measures the time. The time is printed when the object is dropped.
//!
//! ### Examples
//!
//! ```rust
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
extern crate log;

#[derive(Debug)]
pub enum MeasureTimeLogLevel {Info, Debug, Trace, Print}

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
        match self.level  {
            MeasureTimeLogLevel::Info  =>    info!("{} took {}ms ",self.name, time_in_ms),
            MeasureTimeLogLevel::Debug =>   debug!("{} took {}ms ",self.name, time_in_ms),
            MeasureTimeLogLevel::Trace => trace!("{} took {}ms ",self.name, time_in_ms),
            MeasureTimeLogLevel::Print => println!("{} took {}ms ",self.name, time_in_ms),
        }
    }
}


/// logs the time with the info! macro
#[macro_export]
macro_rules! info_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Info); } }
/// logs the time with the debug! macro
#[macro_export]
macro_rules! debug_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Debug); } }
/// logs the time with the trace! macro
#[macro_export]
macro_rules! trace_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Trace); } }
/// logs the time with the print! macro
#[macro_export]
macro_rules! print_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Print); } }


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
