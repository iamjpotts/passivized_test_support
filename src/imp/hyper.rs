use http::Response;
use http_body_util::{BodyExt, Empty};
use hyper::body::{Body, Bytes, Incoming};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use native_tls::TlsConnector;
use crate::http_status_tests::StatusTest;
use crate::http_errors::HttpError;

pub(crate) struct HyperHttp {}

const MAX_IDLE_PER_HOST: usize = 1;

impl HyperHttp {

    fn build_client_http<D: Send, B: Body<Data = D> + Send>() -> Client<HttpConnector, B> {
        Client::builder(TokioExecutor::new())
            .pool_max_idle_per_host(MAX_IDLE_PER_HOST)
            .build(HttpConnector::new())
    }

    fn build_client_from_tls<D: Send, B: Body<Data = D> + Send>(tls: TlsConnector) -> Client<hyper_tls::HttpsConnector<HttpConnector>, B> {
        let mut inner = HttpConnector::new();
        inner.enforce_http(false);

        Client::builder(TokioExecutor::new())
            .pool_max_idle_per_host(MAX_IDLE_PER_HOST)
            .build(hyper_tls::HttpsConnector::from((inner, tls.into())))
    }

    pub(crate) async fn get_text_http<T>(&self, url: &str, status_test: &T) -> Result<String, HttpError>
    where
        T: StatusTest
    {
        let client = Self::build_client_http();

        let request = hyper::http::Request::get(url)
            .body(empty_body())?;

        let response = client
            .request(request)
            .await?;

        Self::parse_response(response, status_test).await
    }

    pub(crate) async fn get_text_https<T>(&self, url: &str, tls: TlsConnector, status_test: &T) -> Result<String, HttpError>
    where
        T: StatusTest
    {
        let client = Self::build_client_from_tls(tls);

        let request = hyper::http::Request::get(url)
            .body(empty_body())?;

        let response = client
            .request(request)
            .await?;

        Self::parse_response(response, status_test).await
    }

    async fn parse_response<T>(response: Response<Incoming>, status_test: &T) -> Result<String, HttpError>
    where
        T: StatusTest
    {
        if status_test.test(response.status()) {
            let response_body = incoming_bytes(response).await?;

            Ok(String::from_utf8(response_body)?)
        }
        else {
            Err(HttpError::Status(response.status()))
        }
    }
}

fn empty_body() -> Empty<Bytes> {
    Empty::new()
}

async fn incoming_bytes(mut response: Response<Incoming>) -> Result<Vec<u8>, hyper::Error> {
    let mut response_body: Vec<u8> = Vec::new();

    while let Some(frame_result) = response.frame().await {
        let frame = frame_result?;

        if let Some(segment) = frame.data_ref() {
            response_body.extend_from_slice(segment.iter().as_slice());
        }
    }

    Ok(response_body)
}

#[cfg(test)]
mod test_get_text_from_http {
    use http::StatusCode;
    use super::HyperHttp;
    use crate::http_errors::HttpError;
    use crate::http_status_tests::is_success;

    #[tokio::test]
    async fn fails_when_server_error() {
        let server = mockito::Server::new_async().await;

        let actual = HyperHttp{}.get_text_http(&format!("{}/qux", server.url()), &is_success())
            .await
            .unwrap_err();

        if let HttpError::Status(status) = actual {
            assert_eq!(StatusCode::NOT_IMPLEMENTED, status);
        }
        else {
            panic!("Unexpected error: {:?}", actual);
        }
    }

    #[tokio::test]
    async fn fails_when_server_not_present() {
        let server = "http://127.0.0.200:1234";

        HyperHttp{}.get_text_http(&format!("{}/foo", server), &is_success())
            .await
            .unwrap_err();
    }

    #[tokio::test]
    async fn gets() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/foo")
            .with_body("bar")
            .create_async()
            .await;

        let actual = HyperHttp{}.get_text_http(&format!("{}/foo", server.url()), &is_success())
            .await
            .unwrap();

        assert_eq!("bar", actual);
    }
}