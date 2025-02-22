
### `okstd::test`

take a function and if it's not async, just add the #[test] attribute
if it's async, add the #[test] attribute and setup the runtime
take the previous function body then pass it into block_on as a closure

```rust
use ok_macros as okstd;
#[okstd::test]
fn does_something() {
  // do something
}
```

to

```rust
#[test]
fn does_something() {
 // do something
}
```

or

```rust
use ok_macros as okstd;

#[okstd::test]
async fn does_something() {
 // do something
}
```

to

```rust
#[test]
fn does_something() {
    Runtimes::setup_runtimes().unwrap().block_on(async {
        // do something
    });
}
```
