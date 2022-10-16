use log::LevelFilter;
use simple_logger::SimpleLogger;

/// Configure some defaults that are sensible for examples and tests.
pub fn enable() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_threads(true)
        .env()
        .init()
        .unwrap();
}
