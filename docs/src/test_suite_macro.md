
### `test_suite!` macro

The `test_suite!` macro provides a convenient way to define a collection of tests that share a common resource with setup and teardown functionality.

#### Syntax

```rust
# use std::{fs, io, path::PathBuf};
# use okstd::{macro_test_suite::Resource, test_suite};
#
# pub struct TempFileResource {
#     path: PathBuf,
# }
#
# impl TempFileResource {
#     pub fn new() -> Self {
#         let temp_dir = std::env::temp_dir();
#         let path = temp_dir.join(format!("okstd_test_{}", std::process::id()));
#         TempFileResource { path }
#     }
#
#     pub fn write_content(&self, content: &str) -> io::Result<()> {
#         fs::write(&self.path, content)
#     }
#
#     pub fn read_content(&self) -> io::Result<String> {
#         fs::read_to_string(&self.path)
#     }
# }
#
# impl Resource for TempFileResource {
#     fn setup() -> Self {
#         TempFileResource::new()
#     }
#
#     fn teardown(&mut self) {
#         if self.path.exists() {
#             fs::remove_file(&self.path).ok();
#         }
#     }
# }
test_suite! {
    TempFileResource => {
        test!(test_name, |resource| {
            // Test body that operates on the resource
            // Must return Result<(), ()>
            Ok(())
        })
        // Additional tests...
    }
}
```

#### Resource Trait

Resources must implement the `Resource` trait:

```rust
pub trait Resource {
    fn setup() -> Self;
    fn teardown(&mut self);
}
```

- `setup()`: Called once to initialize the resource
- `teardown()`: Called after all tests complete to clean up

#### Example Usage

```rust
use std::{fs, path::PathBuf};
use okstd::{macro_test_suite::Resource, test_suite};

pub struct TempFileResource {
    path: PathBuf,
}

impl TempFileResource {
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join(format!("okstd_test_{}", std::process::id()));
        TempFileResource { path }
    }

    pub fn write_content(&self, content: &str) -> std::io::Result<()> {
        fs::write(&self.path, content)
    }

    pub fn read_content(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }
}

impl Resource for TempFileResource {
    fn setup() -> Self {
        TempFileResource::new()
    }

    fn teardown(&mut self) {
        if self.path.exists() {
            fs::remove_file(&self.path).ok();
        }
    }
}

test_suite! {
    TempFileResource => {
        test!(write_and_read, |resource| {
            resource.write_content("Hello, world!").unwrap();
            assert_eq!(resource.read_content().unwrap(), "Hello, world!");
            Ok(())
        })

        test!(another_test, |resource| {
            // This test also operates on the same resource instance
            Ok(())
        })
    }
}
```

#### How It Works

1. The macro generates a static `TestSuite<ResourceType>` wrapped in `LazyLock<Mutex<_>>`
2. For each `test!` declaration, it creates a standard Rust `#[test]` function
3. Each test function adds its body to the test suite's execution queue
4. Tests are executed when the suite is dropped, in reverse order
5. The resource's `teardown()` method is called after all tests complete
