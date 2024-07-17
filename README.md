# pmapi-rs

This Rust crate provides a wrapper for the Performance Metrics API (PMAPI), allowing you to fetch and work with performance metrics in a safe and idiomatic way.

## Usage

Here's a quick example of how to use this wrapper to fetch a raw metric result:

```rust
let context = Context::new(ContextType::Host, "localhost").unwrap();
let metric_id = lookup_name("kernel.all.cpu.user").unwrap();
let result = fetch_raw(metric_id).unwrap();
let vset = unsafe { *(result.vset[0]) };
println!("kernel.all.cpu.user: {}", vset.value);
```
