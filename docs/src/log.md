### `okstd::log`

```rust
#[cfg(feature = "macros")]
{
    use okstd::prelude::*;

    #[okstd::log(debug)]
    fn something() {
        debug!("Hello, world!");
        println!("Hello, world!");
    }
}
```
