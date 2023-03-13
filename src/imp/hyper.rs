use native_tls::TlsConnector;
use crate::http_status_tests::StatusTest;
use crate::http_errors::HttpError;

pub(crate) struct HyperHttp {}

impl HyperHttp {

    fn build_client_http() -> hyper::Client<hyper::client::HttpConnector> {
        hyper::Client::builder()
            .pool_max_idle_per_host(0)
            .build::<_, hyper::Body>(hyper::client::HttpConnector::new())
    }

    fn build_client_from_tls(tls: TlsConnector) -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>> {
        let mut inner = hyper::client::HttpConnector::new();
        inner.enforce_http(false);

        hyper::Client::builder()
            .pool_max_idle_per_host(0)
            .build::<_, hyper::Body>(hyper_tls::HttpsConnector::from((inner, tls.into())))
    }

    pub(crate) async fn get_text_http<T>(&self, url: &str, status_test: &T) -> Result<String, HttpError>
    where
        T: StatusTest
    {
        let client = Self::build_client_http();

        let request: hyper::Request<hyper::Body> = hyper::http::Request::get(url)
            .body(hyper::Body::empty())?;

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

        let request: hyper::Request<hyper::Body> = hyper::http::Request::get(url)
            .body(hyper::Body::empty())?;

        let response = client
            .request(request)
            .await?;

        Self::parse_response(response, status_test).await
    }

    async fn parse_response<T>(response: hyper::Response<hyper::Body>, status_test: &T) -> Result<String, HttpError>
    where
        T: StatusTest
    {
        if status_test.test(response.status()) {
            let response_body = hyper::body::to_bytes(response)
                .await?;

            Ok(String::from_utf8(response_body.into())?)
        }
        else {
            Err(HttpError::Status(response.status()))
        }
    }
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