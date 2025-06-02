// async_runtime.rs

// Removed: use std::future::Future; as it's no longer used directly in this file
// after removing the Runtime trait and its impls.

pub use futures::{
    future::{FutureExt, TryFutureExt},
    sink::{Sink, SinkExt},
    stream::{Stream, StreamExt, TryStreamExt},
};

pub trait AsyncRead: futures::io::AsyncRead {}
impl<T: ?Sized + futures::io::AsyncRead> AsyncRead for T {}

pub trait AsyncWrite: futures::io::AsyncWrite {}
impl<T: ?Sized + futures::io::AsyncWrite> AsyncWrite for T {}

type Result<T> = anyhow::Result<T>;

// Removed: pub trait Runtime: Unpin { ... }
// Removed: impl Runtime for tokio::runtime::Runtime { ... }
// Removed: pub enum Runtimes { ... }
// Removed: impl Runtime for Runtimes { ... }
// Removed: impl Runtimes { pub fn setup_runtimes() -> Result<Runtimes> { ... } }
