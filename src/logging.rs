use log::LevelFilter;
use simple_logger::SimpleLogger;

/// Configure some defaults that are sensible for examples and tests.
pub fn enable() {
    enable_with_level(LevelFilter::Info)
}

pub fn enable_with_level(level: LevelFilter) {
    SimpleLogger::new()
        .with_level(level)
        .with_threads(true)
        .env()
        .init()
        .unwrap();
}
