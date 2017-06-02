# Measure Time in Rust

[![](http://meritbadge.herokuapp.com/fst)](https://crates.io/crates/measure_time)

The macros measures the time until end of scope

This is done by creating an object, which measures the time. The time is printed when the object is dropped.

### Installation

Simply add a corresponding entry to your `Cargo.toml` dependency list:

```toml,ignore
[dependencies]
measure_time = "0.1"
```

And add this to your crate root (lib.rs/main.rs):

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
