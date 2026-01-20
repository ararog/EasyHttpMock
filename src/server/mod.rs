use std::future::Future;

use vetis::{server::errors::VetisError, RequestType, ResponseType};

use crate::errors::EasyHttpMockError;

pub mod adapters;

pub trait ServerAdapter {
    type Config: Clone;

    fn new(config: Self::Config) -> Self;

    fn base_url(&self) -> String;

    fn start<H, Fut>(&mut self, handler: H) -> impl Future<Output = Result<(), EasyHttpMockError>>
    where
        H: Fn(RequestType) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<ResponseType, VetisError>> + Send + 'static;

    fn stop(&mut self) -> impl Future<Output = Result<(), EasyHttpMockError>>;
}

pub trait BaseUrlGenerator<S>
where
    S: ServerAdapter,
    S::Config: Clone,
{
    fn gen_url(&self) -> String;
}

pub trait PortGenerator<S>
where
    S: ServerAdapter,
    S::Config: Clone,
{
    fn random_port() -> u16 {
        rand::random_range(9000..65535)
    }

    fn with_random_port(self) -> Self;
}
