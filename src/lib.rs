use std::future::Future;

use crate::{config::EasyHttpMockConfig, errors::EasyHttpMockError, server::ServerAdapter};

use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;
use hyper::body::Incoming;
use vetis::{server::errors::VetisError, RequestType, ResponseType};

pub mod config;
pub mod errors;
pub mod server;

mod tests;

pub struct EasyHttpMock<S>
where
    S: ServerAdapter,
{
    config: EasyHttpMockConfig<S>,
    server: S,
}

impl<S: ServerAdapter> EasyHttpMock<S> {
    pub fn new(config: EasyHttpMockConfig<S>) -> EasyHttpMock<S> {
        let server = S::new(config.server_config.clone());
        EasyHttpMock { config, server }
    }

    pub fn url(&self, path: &str) -> String {
        if let Some(base_url) = &self.config.base_url {
            format!("{}{}", base_url, path)
        } else {
            format!("{}{}", self.server.base_url(), path)
        }
    }

    pub fn base_url(&self) -> String {
        self.server.base_url()
    }

    pub async fn start<H, Fut>(&mut self, handler: H) -> Result<(), EasyHttpMockError>
    where
        H: Fn(RequestType) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<ResponseType, VetisError>> + Send + 'static,
    {
        self.server.start(handler).await
    }

    pub async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        self.server.stop().await
    }
}

pub fn mock_response(status: StatusCode, body: &[u8]) -> ResponseType {
    http::Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(body.to_vec())))
        .unwrap()
}
