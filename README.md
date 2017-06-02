# rust_measure_time

The macros measures the time until end of scope

This is done by creating an object, which measures the time. The time is printed when the object is dropped.

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
