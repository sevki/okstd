#![cfg(test)]
use std::{fs, io, path::PathBuf};

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

    pub fn write_content(&self, content: &str) -> io::Result<()> {
        fs::write(&self.path, content)
    }

    pub fn read_content(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }
}

impl Default for TempFileResource {
    fn default() -> Self {
        Self::new()
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
        println!("Cleaned up temp file: {:?}", self.path);
    }
}

test_suite! {
    TempFileResource => {
        test!(write_and_read, |resource| {
            resource.write_content("Hello, world!").unwrap();
            assert_eq!(resource.read_content().unwrap(), "Hello, world!");
            Ok(())
        })

        test!(write_and_read_with_setup, |resource| {
            resource.write_content("Hello, world!").unwrap();
            assert_eq!(resource.read_content().unwrap(), "Hello, world!");
            Ok(())
        })
    }
}
