use std::error::Error;
use std::string::FromUtf8Error;
use http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("{0}")]
    FromUtf8(FromUtf8Error),

    #[error("{0}")]
    Http(http::Error),

    #[error("{0}")]
    Other(Box<dyn Error>),

    #[error("{0}")]
    Status(StatusCode)
}

impl From<FromUtf8Error> for HttpError {
    fn from(other: FromUtf8Error) -> Self {
        Self::FromUtf8(other)
    }
}

impl From<http::Error> for HttpError {
    fn from(other: http::Error) -> Self {
        Self::Http(other)
    }
}

impl From<hyper::Error> for HttpError {
    fn from(other: hyper::Error) -> Self {
        Self::Other(Box::new(other))
    }
}

impl From<hyper_util::client::legacy::Error> for HttpError {
    fn from(other: hyper_util::client::legacy::Error) -> Self {
        Self::Other(Box::new(other))
    }
}
