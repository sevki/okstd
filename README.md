# okstd

<img src="okstd.png" align="right" width="200">

Standards that are OK.

## Getting Started

```bash
cargo add okstd@0.1.0
```

```rust
use okstd::prelude::*;
```
## Examples
### `okstd::main`

```rust
#[okstd::main]
async fn main() {
    something();
}
```

### `okstd::log`
```rust
#[okstd::log(debug)]
fn something() {
    debug!("Hello, world!");
    println!("Hello, world!");
}
```

## Experimental Features 

> [!CAUTION]
> Very unstable and only available under `unstable` feature flag.

### `okstd::crashdump`

```rust
#[okstd::log(info)]
#[okstd::crashdump]
#[okstd::main]
async fn main() {
    let a = 0;
    let b = 1;
    let c = b / a;
    panic!("This is a panic");
}
```
will return a crashdump string like so

```text
SourceMap { file: Some("integration/src/panics.rs"), tokens: [], index: [], names: ["backtrace::backtrace::trace_unsynchronized", "backtrace::backtrace::trace", "okstd::notokpanic::panic_hook", "core::ops::function::Fn::call", "<alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call", "std::panicking::rust_panic_with_hook", "std::panicking::begin_panic_handler::{{closure}}", "std::sys_common::backtrace::__rust_end_short_backtrace", "rust_begin_unwind", "core::panicking::panic_fmt", "core::panicking::panic", "panics::old_main::{{closure}}", "tokio::runtime::park::CachedParkThread::block_on::{{closure}}", "tokio::runtime::park::CachedParkThread::block_on", "tokio::runtime::context::blocking::BlockingRegionGuard::block_on", "tokio::runtime::scheduler::multi_thread::MultiThread::block_on::{{closure}}", "tokio::runtime::context::runtime::enter_runtime", "tokio::runtime::scheduler::multi_thread::MultiThread::block_on", "tokio::runtime::runtime::Runtime::block_on", "<okstd::okasync::Runtimes as okstd::okasync::Runtime>::block_on", "panics::main", "core::ops::function::FnOnce::call_once", "std::sys_common::backtrace::__rust_begin_short_backtrace", "std::rt::lang_start::{{closure}}", "std::panicking::try::do_call", "std::panicking::try", "std::panic::catch_unwind", "std::rt::lang_start_internal::{{closure}}", "std::rt::lang_start_internal", "std::rt::lang_start", "main", "_start"], source_root: None, sources: [], sources_prefixed: None, sources_content: [], debug_id: None }
Filename: Ok("/scratch/cargo_target/debug/panics")
Crashdump URL: https://crashdu.mp/🐧/♔/aea3ab2067116e3327bb51dc3bed94cd0/g98qBgj9qBg80qBg7r2Ng7gmOg4nxOgrnxOgt9wOgnmxOg4gpBghhpBggnpBg2spBg7npBg9lpBgp9pBgjsqBgk9pBgwqqBgl+pBgxmpBg/upOg4mqBg4kqBg500Cgnl6Kg6yuDgilvOg500Cgnl6Kg6yuDgohvOgzkqBg9mpBgslpB?cGFuaWMgb2NjdXJyZWQ6IGF0dGVtcHQgdG8gZGl2aWRlIGJ5IHplcm8gYXQgaW50ZWdyYXRpb24vc3JjL3Bhbmljcy5yczoxMToxMw
```

[crashdu.mp/🐧/♔/aea3ab2067116e3327bb51dc3bed94cd0/g98qBgj9qBg80qBg7r2Ng7gmOg4nxOgrnxOgt9wOgnmxOg4gpBghhpBggnpBg2spBg7npBg9lpBgp9pBgjsqBgk9pBgwqqBgl+pBgxmpBg/upOg4mqBg4kqBg500Cgnl6Kg6yuDgilvOg500Cgnl6Kg6yuDgohvOgzkqBg9mpBgslpB?cGFuaWMgb2NjdXJyZWQ6IGF0dGVtcHQgdG8gZGl2aWRlIGJ5IHplcm8gYXQgaW50ZWdyYXRpb24vc3JjL3Bhbmljcy5yczoxMToxMw](https://crashdu.mp/🐧/♔/aea3ab2067116e3327bb51dc3bed94cd0/g98qBgj9qBg80qBg7r2Ng7gmOg4nxOgrnxOgt9wOgnmxOg4gpBghhpBggnpBg2spBg7npBg9lpBgp9pBgjsqBgk9pBgwqqBgl+pBgxmpBg/upOg4mqBg4kqBg500Cgnl6Kg6yuDgilvOg500Cgnl6Kg6yuDgohvOgzkqBg9mpBgslpB?cGFuaWMgb2NjdXJyZWQ6IGF0dGVtcHQgdG8gZGl2aWRlIGJ5IHplcm8gYXQgaW50ZWdyYXRpb24vc3JjL3Bhbmljcy5yczoxMToxMw)