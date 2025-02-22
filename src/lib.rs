pub mod oklog;
// if not wasm
#[cfg(not(target_arch = "wasm32"))]
pub mod okasync;

use log as rustlog;

pub mod prelude {
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::okasync::*;

    pub use crate::oklog::setup_logging;

    #[cfg(feature = "macros")]
    pub use super::impls;
    #[cfg(feature = "macros")]
    pub use super::log;
    #[cfg(feature = "macros")]
    pub use super::main;

    // re-export the slog macros
    pub use {
        crate::rustlog::{debug, error, info, trace, warn},
        fern::*,
    };

    #[cfg(feature = "macros")]
    pub use crate::rustlog::LevelFilter;

    #[cfg(feature = "argh")]
    pub use argh::*;
}

#[cfg(feature = "macros")]
#[doc = include_str!("../docs/src/impls.md")]
pub use ok_macros::impls;
#[cfg(feature = "macros")]
#[doc = include_str!("../docs/src/log.md")]
pub use ok_macros::log;
#[cfg(feature = "macros")]
#[doc = include_str!("../docs/src/main.md")]
pub use ok_macros::main;
#[cfg(feature = "macros")]
#[doc = include_str!("../docs/src/test.md")]
pub use ok_macros::test;
