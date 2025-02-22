
### `okstd::main`

```rust
#[cfg(feature = "macros")]
{
    use okstd::prelude::*;
    fn something() {
        println!("Hello, world!");
    }


    #[okstd::main]
    async fn main() {
        something();
    }
}
```
