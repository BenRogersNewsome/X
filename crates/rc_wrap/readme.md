# Rust Rc Wrap

`rc_wrap` is for creating structs that behave as `Rc`s, without needing to deal directly with `Rc`s, for situations in which you always want a given struct to have shared ownership.

**Example**
```rust
// main.rs

extern crate rc_wrap;
use rc_wrap::rc_wrap;

#[rc_wrap(
    pub WrappedItem
)]
struct RawItem {
    val: u32,
}

let a: WrappedItem = WrappedItem::new(42);
println!("{:?}", a.val); // 42

let b: WrappedItem = a.clone();
println!("{:?}", b.val); // 42
```