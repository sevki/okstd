//!

pub mod log;
pub mod okasync;

#[cfg(feature = "unstable")]
pub mod notokpanic;
mod e2e_tests;

extern crate fern;

pub mod prelude {
    pub use crate::okasync::*;

    pub use crate::log::setup_logging;
    
    pub use super::main;
    // re-export the slog macros
    pub use fern::*;
    pub use log::debug;
    pub use log::error;
    pub use log::info;
    pub use log::trace;
    pub use log::warn;

    pub use log::LevelFilter;

    pub use std::panic::set_hook;
    
    #[cfg(feature = "unstable")]
    pub use crate::notokpanic::panic_hook;
}

pub use ok_macros::main;
pub use ok_macros::test;
pub use ok_macros::log;

#[cfg(feature = "unstable")]
pub use ok_macros::crashdump;