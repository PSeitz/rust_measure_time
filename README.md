# measure_time

[![](http://meritbadge.herokuapp.com/measure_time)](https://crates.io/crates/measure_time)

The crate provides macros, which measure the time until end of scope.

This is done by creating an object, which measures the time. The time is printed when the object is dropped.

### Installation

Simply add a corresponding entry to your `Cargo.toml` dependency list:

```toml,ignore
[dependencies]
measure_time = "0.4"
log = "0.4"
```

And add this to your crate root (e.g. lib.rs/main.rs):

```rust,ignore
#[macro_use]
extern crate measure_time;
#[macro_use]
extern crate log;
```


### Examples

```rust
#[macro_use]
extern crate measure_time;
#[macro_use]
extern crate log;
fn main() {
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
```

### Changelog

#### Version 0.4
Objects to measure time are only created when the log level is enabled, else ```None``` will be created

#### Version 0.4.2
Add error and warn levels

#### Version 0.5.0
change time formatting for improved readability