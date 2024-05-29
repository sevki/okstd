pub mod oklog;
// if not wasm
#[cfg(not(target_arch = "wasm32"))]
pub mod okasync;

#[cfg(feature = "unstable")]
pub mod okpanic;
mod e2e_tests;

extern crate fern;

use log as rustlog;

pub mod prelude {
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::okasync::*;

    pub use crate::oklog::setup_logging;
    
    #[cfg(feature = "macros")]    
    pub use super::main;
    #[cfg(feature = "macros")]
    pub use super::log;

    // re-export the slog macros
    pub use fern::*;
    pub use crate::rustlog::debug;
    pub use crate::rustlog::error;
    pub use crate::rustlog::info;
    pub use crate::rustlog::trace;
    pub use crate::rustlog::warn;

    pub use crate::rustlog::LevelFilter;
    #[cfg(feature = "macros")]
    pub use std::panic::set_hook;
    #[cfg(feature = "macros")]    
    #[cfg(feature = "unstable")]
    pub use crate::okpanic::panic_hook;
}

#[cfg(feature = "macros")]
pub use ok_macros::main;
#[cfg(feature = "macros")]
pub use ok_macros::test;
#[cfg(feature = "macros")]
pub use ok_macros::log;


#[cfg(feature = "macros")]
#[cfg(feature = "unstable")]
pub use ok_macros::crashdump;