use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use sourcemap::SourceMapBuilder;
use std::{backtrace as stdbt, env, panic::PanicInfo};
use symbolic::{
    common::{DebugId, Uuid},
    debuginfo::ObjectLike,
};

/// Checks if a function is considered to be not in-app
pub fn is_sys_function(func: &str) -> bool {
    WELL_KNOWN_SYS_MODULES.iter().any(|m| func.contains(m))
}

/// Checks if a function is a well-known system function
#[allow(dead_code)]
fn is_well_known(func: &str) -> bool {
    WELL_KNOWN_BORDER_FRAMES.iter().any(|m| func.starts_with(m))
}

const WELL_KNOWN_SYS_MODULES: &[&str] = &[
    "std::",
    "core::",
    "alloc::",
    "backtrace::",
    "sentry::",
    "sentry_core::",
    "sentry_types::",
    // these are not modules but things like __rust_maybe_catch_panic
    "__rust_",
    "___rust_",
    // these are well-known library frames
    "anyhow::",
    "log::",
    "tokio::",
    "tracing_core::",
];

#[allow(dead_code)]
const WELL_KNOWN_BORDER_FRAMES: &[&str] = &[
    "std::panicking::begin_panic",
    "core::panicking::panic",
    // well-known library frames
    "anyhow::",
    "<sentry_log::Logger as log::Log>::log",
    "tracing_core::",
];

// i64 to u64 without loss of data
fn u64(x: i64) -> u64 {
    x as u64
}

// i64 to u64 without loss of data
fn i64(x: u64) -> i64 {
    x as i64
}

fn uuid_to_i64(uuid: Uuid) -> (i64, i64) {
    let (high, low) = uuid.as_u64_pair();
    (
        i64::from_be_bytes(high.to_ne_bytes()),
        i64::from_ne_bytes(low.to_ne_bytes()),
    )
}

fn i64_to_uuid(high: i64, low: i64) -> Uuid {
    let high = u64::from_ne_bytes(high.to_ne_bytes());
    let low = u64::from_ne_bytes(low.to_ne_bytes());
    Uuid::from_u64_pair(high, low)
}

// generate_crashdump_url generates a crashdump URL for the
// given addresses, current platform, architecture and debugId
// buildId, commit and cargo package meta.
fn encode_crashdump_url(addresses: &[i64], debug_id: DebugId) -> String {
    let platform = match std::env::consts::OS {
        "linux" => "🐧",
        "bldy" => "👷",
        "oklinux" => "👌",
        // browser emoji
        "wasi" => "🌐",
        _ => "❓",
    };
    let arch = match std::env::consts::ARCH {
        "x86" => "✖️",
        "x86_64" => "♔", // chess board has 64squares
        "aarch64" => "🦾",
        "arm" => "💪",
        "wasm32" => "🕸️",
        _ => "unknown",
    };

    println!("Filename: {:?}", env::current_exe());
    let addrs = sourcemap::vlq::generate_vlq_segment(addresses).unwrap();
    let mut pathbuf = std::path::PathBuf::new();
    pathbuf.push(platform);
    pathbuf.push(arch);
    pathbuf.push(debug_id.breakpad().to_string().to_lowercase());
    pathbuf.push(addrs);
    format!("https://crashdu.mp/{}", pathbuf.to_str().unwrap())
}

// decode_crashdump_url decodes a crashdump URL and returns the
// addresses, debugId, buildId, commit and cargo package meta.
fn decode_crashdump_url(url: &str) -> (Vec<i64>, DebugId) {
    let u = url::Url::parse(url).unwrap();
    let segments = u.path_segments().unwrap().collect::<Vec<_>>();
    let x = segments[2].to_uppercase();
    let x = x.as_str();
    let debug_id = DebugId::from_breakpad(x).expect("da");
    let addrs = sourcemap::vlq::parse_vlq_segment(segments[3]).unwrap();
    (addrs, debug_id)
}

pub fn panic_hook(info: &PanicInfo) {
    let location = info.location().unwrap();

    let mut builder = SourceMapBuilder::new(Some(location.file()));

    let mut addrs: Vec<i64> = Vec::new();

    let data = std::fs::read(env::current_exe().unwrap()).unwrap();

    let obj = symbolic::debuginfo::Object::parse(data.as_slice()).unwrap();

    let symbols = obj.symbols().collect::<Vec<_>>();

    let _bt = stdbt::Backtrace::force_capture();

    let finder = |name: &str| {
        symbols.iter().find_map(|symbol| {
            let nname = symbolic::demangle::demangle(symbol.name().unwrap());
            // println!("Found symbol: {}, {}", name, nname);
            if nname == name {
                Some(symbol)
            } else {
                None
            }
        })
    };

    let debug_id = obj.debug_id();

    backtrace::trace(|frame| {
        let _ip = frame.ip();
        let symbol_address = frame.symbol_address();

        let _addr = symbol_address as i64;
        backtrace::resolve_frame(frame, |symbol| {
            let name = symbol
                .name()
                .map_or("<unknown>", |name| name.as_str().unwrap());
            let name = symbolic::demangle::demangle(name);
            let found = finder(name.to_string().as_str());
            if let Some(symbol) = found {
                addrs.push(symbol.address.try_into().unwrap());
                let name = name.to_string();
                builder.add_name(name.as_str());
            }

            // addrs.push(addr);
        });

        true // keep going to the next frame
    });
    println!("{:?}", builder.into_sourcemap());
    let message = info.payload().downcast_ref::<&str>().unwrap();
    let msgggg = format!("panic occurred: {} at {}", message, location);
    let url = encode_crashdump_url(&addrs, debug_id);
    let mut str = String::new();
    let _x = &URL_SAFE_NO_PAD.encode_string(msgggg, &mut str);
    println!("Crashdump URL: {}?{}", url, str);
}

#[cfg(test)]
mod tests {
    

    #[crate::test]
    #[should_panic]
    fn test_panic() {
        panic!("this is a panic message");
    }
    use super::*;

    #[test]
    fn test_encode_crashdump_url() {
        let addresses = vec![14, 02, 1988];
        let debug_id =
            DebugId::from_uuid(Uuid::parse_str("08ab7650-ed55-4006-b665-867495ba85c5").unwrap());
        let url = encode_crashdump_url(&addresses, debug_id);
        assert_eq!(
            url,
            "https://crashdu.mp/🐧/♔/08ab7650ed554006b665867495ba85c50/2HwcqxB"
        );
    }

    #[test]
    fn test_decode_crashdump_url() {
        let url = "https://crashdu.mp/🐧/♔/08ab7650ed554006b665867495ba85c50/2HwcqxB";
        let (addresses, debug_id) = decode_crashdump_url(url);
        assert_eq!(addresses, vec![123, 456, 789]);
        assert_eq!(
            debug_id,
            DebugId::from_uuid(Uuid::parse_str("08ab7650-ed55-4006-b665-867495ba85c5").unwrap())
        );
    }
}
