use std::borrow::Borrow;
use native_tls::TlsConnector;
use crate::http_errors::HttpError;
use crate::http_status_tests::{is_success, StatusTest};
use crate::imp::hyper::HyperHttp;

// In the future this will be conditional compiled to
// different implementations based on crate features.
static HTTP_IMP: HyperHttp = HyperHttp {};

/// Get text response from a server that is not using TLS/HTTPS.
pub async fn get_text_http<U>(url: U) -> Result<String, HttpError>
where
    U: Borrow<str>
{
    get_text_http_with(url.borrow(), is_success()).await
}

pub(crate) async fn get_text_http_with<U, T, BT>(url: U, status_test: BT) -> Result<String, HttpError>
where
    U: Borrow<str>,
    T: StatusTest,
    BT: Borrow<T>
{
    HTTP_IMP.get_text_http(url.borrow(), status_test.borrow()).await
}

/// Get text response from a server that is using TLS/HTTPS.
pub async fn get_text_https<U>(url: U, tls: TlsConnector) -> Result<String, HttpError>
where
    U: Borrow<str>
{
    get_text_https_with(url, tls, is_success()).await
}

pub (crate) async fn get_text_https_with<U, T, BT>(url: U, tls: TlsConnector, status_test: BT) -> Result<String, HttpError>
where
    U: Borrow<str>,
    T: StatusTest,
    BT: Borrow<T>
{
    HTTP_IMP.get_text_https(url.borrow(), tls, status_test.borrow()).await
}
