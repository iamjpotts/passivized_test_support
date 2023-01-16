use log::{info, LevelFilter, SetLoggerError};
use simple_logger::SimpleLogger;

/// Configure some defaults that are sensible for examples and tests.
pub fn enable() {
    try_enable()
        .unwrap();
}

/// If a logger has already been configured, does nothing.
pub fn enable_idempotent() {
    match try_enable() {
        Ok(()) => {},
        Err(SetLoggerError { .. }) => {
            info!("Logger already configured.");
        }
    }
}

pub fn enable_with_level(level: LevelFilter) {
    try_enable_with_level(level)
        .unwrap()
}

fn try_enable() -> Result<(), SetLoggerError> {
    try_enable_with_level(LevelFilter::Info)
}

fn try_enable_with_level(level: LevelFilter) -> Result<(), SetLoggerError> {
    SimpleLogger::new()
        .with_level(level)
        .with_threads(true)
        .env()
        .init()
}

#[cfg(test)]
mod test_enable {
    use crate::logging::enable_idempotent;

    #[test]
    fn idempotent() {
        enable_idempotent();
        enable_idempotent();
    }
}
