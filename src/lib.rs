#![doc = include_str!("../docs/src/intro.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/sevki/okstd/refs/heads/main/okstd.png")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sevki/okstd/refs/heads/main/okstd.png"
)]
#![doc(issue_tracker_base_url = "https://issue.is")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod oklog;

#[cfg(feature = "argh")]
extern crate argh;
#[cfg(feature = "macros")]
extern crate ok_macros;
#[cfg(feature = "macros")]
extern crate tokio_macros;
extern crate tracing;

pub mod prelude {

    pub use crate::oklog::setup_logging;

    #[cfg(feature = "macros")]
    pub use super::impls;
    #[cfg(feature = "macros")]
    pub use super::log;
    #[cfg(feature = "macros")]
    pub use super::main;

    pub use tracing::{debug, error, info, trace, warn};

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
pub use tokio_macros::main;
#[cfg(feature = "macros")]
#[doc = include_str!("../docs/src/test.md")]
pub use tokio_macros::test;
