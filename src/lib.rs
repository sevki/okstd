//!

pub mod oklog;
pub mod okasync;

#[cfg(feature = "unstable")]
pub mod notokpanic;
mod e2e_tests;

extern crate fern;

use log as rustlog;

pub mod prelude {
    pub use crate::okasync::*;

    pub use crate::oklog::setup_logging;
    
    pub use super::main;
    pub use super::log;

    // re-export the slog macros
    pub use fern::*;
    pub use crate::rustlog::debug;
    pub use crate::rustlog::error;
    pub use crate::rustlog::info;
    pub use crate::rustlog::trace;
    pub use crate::rustlog::warn;

    pub use crate::rustlog::LevelFilter;

    pub use std::panic::set_hook;
    
    #[cfg(feature = "unstable")]
    pub use crate::notokpanic::panic_hook;
}

pub use ok_macros::main;
pub use ok_macros::test;
pub use ok_macros::log;

#[cfg(feature = "unstable")]
pub use ok_macros::crashdump;