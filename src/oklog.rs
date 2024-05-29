use colored::*;
use fern::colors::ColoredLevelConfig;
use std::{io, path::PathBuf};

// get module color hashes the module name
// and attempts to return a unique color as far as ansi colors go
fn get_module_color(module: &str) -> colored::Color {
    let hash = module
        .chars()
        .fold(0 as u32, |acc, c| acc.wrapping_add(c as u32));
    match hash % 13 {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Blue,
        4 => Color::Magenta,
        5 => Color::Cyan,
        8 => Color::BrightRed,
        9 => Color::BrightGreen,
        10 => Color::BrightYellow,
        11 => Color::BrightBlue,
        12 => Color::BrightMagenta,
        _ => Color::BrightCyan,
    }
}

pub fn setup_logging(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::Green)
        .debug(fern::colors::Color::Blue)
        .trace(fern::colors::Color::Cyan);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let module = record.target().replace("::", "/");
            let module_color = get_module_color(&module);

            let location_buffer = PathBuf::from(record.file().unwrap())
                .canonicalize()
                .unwrap_or(record.file().unwrap().into());
            let loc = location_buffer.to_str().unwrap();

            out.finish(format_args!(
                "{} {}:{} {} {}",
                format!("{}/{}", "ok.software", module).color(module_color),
                loc,
                record.line().unwrap(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(io::stdout())
        .apply()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[test]
    fn test_module_color() {
        let module = "okstd::oklog";
        let color = get_module_color(module);
        println!("Module: {} Color: {:?}", module, color);
    }

    #[test]
    fn test_module_color_randomness_by_hash() {
        let mut colors = vec![];
        for i in 0..100 {
            let module = format!("okstd::oklog::{}", i);
            let color = get_module_color(&module);
            colors.push(color);
        }
        println!("Colors: {:?}", colors);
    }

    #[test]
    fn test_logging() {
        setup_logging(log::LevelFilter::Error).unwrap();
        for var in std::env::vars() {
            info!("{}={}", var.0, var.1);
        }
    }
}
