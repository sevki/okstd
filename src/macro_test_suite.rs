use std::sync::LazyLock;

use crate::BoxPin;

type TestFn<T> = dyn FnOnce(BoxPin<&mut T>) -> Result<(), ()> + Send + 'static;

pub trait Resource {
    fn setup() -> Self;
    fn teardown(&mut self);
}

pub struct TestSuite<T: Resource> {
    t: LazyLock<T>,
    execute: Vec<Box<TestFn<T>>>,
}

impl<T: Resource> TestSuite<T> {
    pub fn new(f: fn() -> T) -> Self {
        TestSuite {
            t: LazyLock::new(f),
            execute: Vec::new(),
        }
    }
}

impl<T: Resource> TestSuite<T> {
    pub fn test(&mut self, f: impl FnOnce(BoxPin<&mut T>) -> Result<(), ()> + Send + 'static) {
        self.execute.push(Box::new(f));
    }
}

impl<T: Resource> Drop for TestSuite<T> {
    fn drop(&mut self) {
        self.execute.reverse();
        for f in self.execute.drain(..) {
            f(Box::pin(&mut self.t)).ok();
        }
        self.t.teardown();
    }
}
#[doc = include_str!("../docs/src/test_suite_macro.md")]
#[macro_export]
macro_rules! test_suite {
    ($resource_type:ident => {
        $(test!($test_name:ident, $body:expr))*
    }) => {
        use std::sync::{LazyLock, Mutex};
        use $crate::test::TestSuite;
        use $crate::test::gtest;


        static SUITE: LazyLock<Mutex<TestSuite<$resource_type>>> =
            LazyLock::new(|| Mutex::new(TestSuite::new($resource_type::setup)));

        $(
            #[gtest]
            #[test]
            fn $test_name() {
                SUITE.lock().unwrap().test($body);
            }
        )*
    };
}
