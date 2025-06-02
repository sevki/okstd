// This file acts as a compile-time check for the main macro.
// It's not intended to be run as a binary with arguments directly via `cargo run`.
// `cargo test` will compile it as part of the test suite.

use ok_macros::main;
// It might be necessary to bring tokio into scope if the macro expansion needs it
// and it's not automatically available. The current macro expansion fully qualifies
// tokio::runtime::Runtime, so this explicit use should not be strictly necessary
// for the macro expansion itself, but it's good practice if the test code
// were to use tokio primitives directly.
// use tokio;

// Dummy Error type for the test
#[derive(Debug)]
struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implementing std::error::Error is good practice for error types.
impl std::error::Error for MyError {}

// This is the function we are testing the macro with.
// The macro will generate a new `fn main(...)` that calls this one (renamed).
#[main]
async fn some_main_function(count: i32, name: String) -> Result<String, MyError> {
    // These print_lns won't show during `cargo test` as this function isn't called directly by the test runner.
    // Their purpose here is just to have some valid async code that uses the arguments.
    println!("Mock processing: count: {}, name: {}", count, name);
    if count > 0 {
        Ok(format!("Processed: {}, {}.", name, count))
    } else {
        Err(MyError("Count was not positive".to_string()))
    }
}

// To make this file part of the test suite and not a main binary:
// We add a normal test function. The compilation of the `some_main_function`
// with the `#[main]` macro is the primary test here.
#[test]
fn main_macro_compiles_with_signature() {
    // This test function doesn't need to do much.
    // Its existence ensures `cargo test` compiles this file.
    // If the `#[main]` macro expansion on `some_main_function` is incorrect
    // (e.g., due to signature mismatches or issues with handling generics/lifetimes),
    // this file will fail to compile, and thus the test `main_macro_compiles_with_signature` will fail.
    assert!(true, "Compile-time test for main macro passed (if this runs, it means the file compiled successfully).");
}
