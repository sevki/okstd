# okstd

<img src="okstd.png" align="right" width="200">

Standards that are OK.

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
#[okstd::main]
async fn main() {
    something();
}
```
## Experimental Features
### `okstd::log`
```rust
#[okstd::log(debug)]
fn something() {
    debug!("Hello, world!");
    println!("Hello, world!");
}
```