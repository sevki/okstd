// async_runtime.rs
use std::future::Future;

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

pub trait Runtime: Unpin {
    fn block_on<F: Future>(&self, future: F) -> F::Output;

    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.block_on(future);
    }
}

impl Runtime for tokio::runtime::Runtime {
    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.block_on(future)
    }
}
#[cfg(feature = "cros_async")]
impl Runtime for cros_async::Executor {
    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.run_until(future).unwrap()
    }
}

pub enum Runtimes {
    Tokio(tokio::runtime::Runtime),

    #[cfg(feature = "cros_async")]
    Cros(cros_async::Executor),
}

impl Runtime for Runtimes {
    fn block_on<F: Future>(&self, future: F) -> F::Output {
        match self {
            Runtimes::Tokio(rt) => rt.block_on(future),

            #[cfg(feature = "cros_async")]
            Runtimes::Cros(executor) => executor.block_on(future),
        }
    }
}

impl Runtimes {
    pub fn setup_runtimes() -> Result<Runtimes> {
        let worker_count = num_cpus::get() * 1.5 as usize;
        if let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
            // create a thread pool with core count of the machine
            .worker_threads(worker_count)
            .enable_all()
            .build()
        {
            Ok(Runtimes::Tokio(rt))
        } else {
            Err(anyhow::anyhow!("No supported runtime available"))
        }
    }
}
