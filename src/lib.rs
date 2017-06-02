//! This is documentation for the `measure_time` crate.
//!
//! The macros measures the time until end of scope
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
//!     }
//!     trace_time!(format!("{:?}", "yep"));
//!     print_time!("yep2");
//! }
//! ```
//!

#[macro_use]
extern crate log;

#[derive(Debug)]
pub enum MeasureTimeLogLevel {Info, Debug, Print}

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
            MeasureTimeLogLevel::Print => println!("{} took {}ms ",self.name, time_in_ms),
        }
    }
}



#[macro_export]
macro_rules! info_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Info); } }
#[macro_export]
macro_rules! debug_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Debug); } }
#[macro_export]
macro_rules! trace_time {($e:expr) => {#[allow(unused_variables)] let time = $crate::MeasureTime::new($e, $crate::MeasureTimeLogLevel::Print); } }
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
