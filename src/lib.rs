pub mod oklog;
// if not wasm
#[cfg(not(target_arch = "wasm32"))]
pub mod okasync;

mod e2e_tests;
#[cfg(feature = "unstable")]
pub mod okpanic;

use log as rustlog;

pub mod prelude {
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::okasync::*;

    pub use crate::oklog::setup_logging;

    #[cfg(feature = "macros")]
    pub use super::log;
    #[cfg(feature = "macros")]
    pub use super::main;

    // re-export the slog macros
    pub use crate::rustlog::debug;
    pub use crate::rustlog::error;
    pub use crate::rustlog::info;
    pub use crate::rustlog::trace;
    pub use crate::rustlog::warn;
    pub use fern::*;

    #[cfg(feature = "macros")]
    #[cfg(feature = "unstable")]
    pub use crate::okpanic::panic_hook;
    pub use crate::rustlog::LevelFilter;
    #[cfg(feature = "macros")]
    pub use std::panic::set_hook;

    #[cfg(feature = "argh")]
    pub use argh::*;
}

#[cfg(feature = "macros")]
pub use ok_macros::log;
#[cfg(feature = "macros")]
pub use ok_macros::main;
#[cfg(feature = "macros")]
pub use ok_macros::test;

#[cfg(feature = "macros")]
#[cfg(feature = "unstable")]
pub use ok_macros::crashdump;
