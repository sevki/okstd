//!

pub mod log;
pub mod okasync;
pub mod notokpanic;
mod e2e_tests;

extern crate slog;

pub mod prelude {
    pub use crate::okasync::*;

    pub use crate::log::setup_logging;
    
    pub use super::main;
    // re-export the slog macros
    pub use slog_scope::crit;
    pub use slog_scope::debug;
    pub use slog_scope::error;
    pub use slog_scope::info;
    pub use slog_scope::trace;
    pub use slog_scope::warn;
    pub use slog_scope::scope;
    pub use slog_scope::logger;
    pub use slog::OwnedKV;
    pub use slog::o;

    pub use slog_scope::set_global_logger;

    pub use std::panic::set_hook;

    pub use crate::notokpanic::panic_hook;
}

pub use ok_macros::main;
pub use ok_macros::test;
pub use ok_macros::log;