//!

pub mod log;
pub mod okasync;

#[cfg(feature = "unstable")]
pub mod notokpanic;
mod e2e_tests;

extern crate fern;
extern crate log as rustlog;

pub mod prelude {
    pub use crate::okasync::*;

    pub use crate::log::setup_logging;
    
    pub use super::main;
    // re-export the slog macros
    pub use fern::*;
    pub use rustlog::debug;
    pub use rustlog::error;
    pub use rustlog::info;
    pub use rustlog::trace;
    pub use rustlog::warn;

    pub use rustlog::LevelFilter;

    pub use std::panic::set_hook;
    
    #[cfg(feature = "unstable")]
    pub use crate::notokpanic::panic_hook;
}

pub use ok_macros::main;
pub use ok_macros::test;
pub use ok_macros::log;

#[cfg(feature = "unstable")]
pub use ok_macros::crashdump;