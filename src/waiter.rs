use std::borrow::Borrow;
use std::time::Duration;
use backoff::backoff::Constant;
use backoff::future::retry_notify;
use log::{info, warn};
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::time::sleep;

use crate::http_errors::HttpError;
use crate::http_status_tests::StatusTest;
use crate::retry::Limit;

/// Wait for the server to be available and the URL to return an HTTP status
/// code that meets an expectation.
///
/// # Example
///
/// ```rust
/// use std::error::Error;
/// use passivized_test_support::http_status_tests::is_success;
/// use passivized_test_support::waiter::wait_for_http_server;
///
/// async fn example() -> Result<(), Box<dyn Error>> {
///     wait_for_http_server("http://foo/missing", is_success()).await;
///     Ok(())
/// }
/// ```
pub async fn wait_for_http_server<U, T>(url: U, status_test: T) -> Result<String, HttpError>
where
    U: Borrow<str>,
    T: StatusTest + Copy,
{
    let borrowed = url.borrow();

    info!("Will wait until can connect to {}", borrowed);

    let interval = Duration::from_secs(2);

    sleep(interval).await;

    retry_notify(
        Limit::new(7, Constant::new(interval)),
        || async {
            super::http::get_text_http_with(borrowed, status_test)
                .await
                .map_err(backoff::Error::transient)
        },
        |error, _| warn!("Retrying after failure: {:?}", error)
    ).await
}

/// Wait for the server to be available and the URL to return an HTTP status
/// code that meets an expectation.
///
/// # Example
///
/// ```rust
/// use std::error::Error;
/// use native_tls::TlsConnector;
/// use passivized_test_support::http_status_tests::is_success;
/// use passivized_test_support::waiter::wait_for_https_server;
///
/// async fn example() -> Result<(), Box<dyn Error>> {
///     wait_for_https_server("http://foo/missing", TlsConnector::new()?, is_success()).await;
///     Ok(())
/// }
/// ```
pub async fn wait_for_https_server<U, T>(url: U, tls: TlsConnector, status_test: T) -> Result<String, HttpError>
where
    U: Borrow<str>,
    T: StatusTest + Copy
{
    let borrowed = url.borrow();

    info!("Will wait until can connect to {}", borrowed);

    let interval = Duration::from_secs(2);

    sleep(interval).await;

    retry_notify(
        Limit::new(5, Constant::new(interval)),
        || async {
            super::http::get_text_https_with(borrowed, tls.clone(), status_test)
                .await
                .map_err(backoff::Error::transient)
        },
        |error, _| warn!("Retrying after failure: {:?}", error)
    ).await
}

pub async fn connect_tcp_server(host: &str, port: u16) -> Result<(), std::io::Error> {
    info!("Will wait until can connect to {}:{}", host, port);

    let connection = TcpStream::connect(format!("{}:{}", host, port)).await?;
    drop(connection);
    Ok(())
}

pub async fn wait_for_tcp_server(host: &str, port: u16) -> Result<(), std::io::Error> {
    let interval = Duration::from_secs(2);

    sleep(interval).await;

    retry_notify(
        Limit::new(4, Constant::new(interval)),
        || async {
            connect_tcp_server(host, port)
                .await
                .map_err(backoff::Error::transient)
        },
        |error, _| warn!("Retrying after failure: {:?}", error)
    ).await
}

#[cfg(test)]
mod test_wait_for_http_server {
    use http::StatusCode;
    use serial_test::serial;
    use crate::http_status_tests::{equals, is_success};
    use crate::waiter::wait_for_http_server;

    #[tokio::test]
    #[serial]
    async fn waits_for_expected_failure() {
        mockito::reset();

        let server = mockito::server_url();

        let _m = mockito::mock("GET", "/abc")
            .with_status(404)
            .with_body("xyz")
            .create();

        let actual = wait_for_http_server(format!("{}/abc", server), equals(StatusCode::NOT_FOUND))
            .await
            .unwrap();

        assert_eq!("xyz", actual);
    }

    #[tokio::test]
    #[serial]
    async fn waits_for_success() {
        mockito::reset();

        let server = mockito::server_url();

        let _m = mockito::mock("GET", "/fizz")
            .with_body("buzz")
            .create();

        let actual = wait_for_http_server(format!("{}/fizz", server), is_success())
            .await
            .unwrap();

        assert_eq!("buzz", actual);
    }
}