# okstd

<img src="https://assets.ok.software/okstd.png" align="right" width="200">

Standards that are OK.

## Motivation

Rust's ecosystem is known for its vibrant community and wealth of high-quality crates. However, this abundance has led to some fragmentation, especially when it comes to foundational aspects like asynchronous programming, I/O, and logging. Developers are often faced with choosing between multiple ways of handling async code (e.g., async-std, tokio, smol, surf), several approaches to async I/O, and numerous logging frameworks (e.g., log, env_logger, pretty_env_logger, femme, flexi_logger). While having options is valuable, it can also lead to decision paralysis and make it harder to build an ecosystem of interoperable libraries and frameworks.

This is the crate when we reach for when we need such primitives, essentially the "Battery Included" crate.

## Getting Started

```bash
cargo add okstd@0.1.0
```

```rust
use okstd::prelude::*;
```

## Examples

### `okstd::main`

```rust
#[cfg(feature = "macros")]
{
    fn something() {
        println!("Hello, world!");
    }
    #[okstd::main]
    async fn main() {
        something();
    }
}
```
