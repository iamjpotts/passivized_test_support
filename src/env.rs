use std::env::VarError;
use std::ffi::OsStr;
use std::fmt::Display;

use log::info;

/// Get the value of a non-optional environment variable.
///
/// Log any error that occurs, or if the variable is missing.
pub fn require_var<A>(name: A) -> Result<String, String>
where
    A: Display + AsRef<OsStr>
{
    var(&name)?
        .ok_or(format!("Env var {} not found", name))
}

/// Get the value of an optional environment variable.
///
/// Log any error that occurs, or if the variable is missing.
pub fn var<A>(name: A) -> Result<Option<String>, String>
where
    A: Display + AsRef<OsStr>
{
    info!("Reading env var {}", name);

    match std::env::var(&name) {
        Err(e) => {
            match e {
                VarError::NotPresent => {
                    info!("Env var {} not found", name);
                    Ok(None)
                }
                VarError::NotUnicode(_) => {
                    Err(format!("Value for environment variable {} was not Unicode", name))
                }
            }
        }
        Ok(value) => {
            Ok(Some(value))
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    const EXISTS: &str = "PATH";

    #[cfg(not(windows))]
    const EXISTS: &str = "HOME";

    mod test_require_var {
        use super::EXISTS;
        use super::super::require_var;

        #[test]
        fn missing() {
            let actual = require_var("DOES_NOT_EXIST")
                .unwrap_err();

            assert_eq!("Env var DOES_NOT_EXIST not found", actual);
        }

        #[test]
        fn present() {
            let expected = std::env::var(EXISTS)
                .unwrap();

            let actual = require_var(EXISTS)
                .unwrap();

            assert_eq!(expected, actual);
        }
    }

    mod test_var {
        use super::EXISTS;
        use super::super::var;

        #[test]
        fn missing() {
            let actual = var("DOES_NOT_EXIST")
                .unwrap();

            assert_eq!(None, actual);
        }

        #[test]
        fn present() {
            let expected = std::env::var(EXISTS)
                .unwrap();

            let actual = var(EXISTS)
                .unwrap();

            assert_eq!(Some(expected), actual);
        }
    }
}