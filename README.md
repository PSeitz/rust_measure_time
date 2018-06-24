# measure_time

[![](http://meritbadge.herokuapp.com/measure_time)](https://crates.io/crates/measure_time)

The crate provides macros, which measure the time until end of scope.

This is done by creating an object, which measures the time. The time is printed when the object is dropped.

The logging behaviour is the same as other log macros like info!(..)

### Installation

Simply add a corresponding entry to your `Cargo.toml` dependency list:

```toml,ignore
[dependencies]
measure_time = "0.6"
```

And add this to your crate root (e.g. lib.rs/main.rs):

```rust,ignore
#[macro_use]
extern crate measure_time;
```


### Examples

```rust
#[macro_use]
extern crate measure_time;
fn main() {
    info_time!("measure function");
    {
        debug_time!("{:?}", "measuring block");
        let mut sum = 0;
        for el in 0..50000 {
            sum+=el;
        }
        println!("{:?}", sum);
    }
    trace_time!("{:?}", "trace");
    print_time!("print");
    error_time!(target: "measure_time", "custom target");
}
```

### Changelog

#### Version 0.4
Objects to measure time are only created when the log level is enabled, else ```None``` will be created

#### Version 0.4.2
Add error and warn levels

#### Version 0.5.0
Change time formatting for improved readability

#### Version 0.6.0
Behaviour is now the same as other log macros (eg. info!). Reexporting log crate macros via pub use.
Previously all tracing was made to the measure_time target (e.g. RUST_LOG=measure_time=debug). This is now fixed.
Added a small example (https://github.com/PSeitz/rust_measure_time/tree/master/measure_time_example).