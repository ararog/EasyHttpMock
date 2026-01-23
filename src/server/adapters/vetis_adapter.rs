use std::future::Future;

use vetis::{
    server::{
        config::{ServerConfig, ServerConfigBuilder},
        errors::VetisError,
    },
    RequestType, ResponseType, Vetis,
};

use crate::{
    config::{EasyHttpMockConfig, EasyHttpMockConfigBuilder},
    errors::{EasyHttpMockError, ServerError},
    server::{BaseUrlGenerator, PortGenerator, ServerAdapter},
    EasyHttpMock,
};

pub struct VetisServerAdapter {
    server: Vetis,
}

impl BaseUrlGenerator<VetisServerAdapter> for &ServerConfig {
    fn gen_url(&self) -> String {
        if self.security().is_some() {
            format!("https://localhost:{}", self.port())
        } else {
            format!("http://localhost:{}", self.port())
        }
    }
}

impl PortGenerator<VetisServerAdapter> for ServerConfigBuilder {
    fn with_random_port(self) -> Self {
        let port = rand::random_range(9000..65535);
        self.port(port)
    }
}

impl Default for EasyHttpMockConfig<VetisServerAdapter> {
    fn default() -> Self {
        let port = 80;
        let server_config = ServerConfig::builder()
            .port(port)
            .interface("0.0.0.0".to_string())
            .build();
        EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(server_config.clone())
            .base_url(format!("http://localhost:{}", port).into())
            .build()
    }
}

impl Default for EasyHttpMock<VetisServerAdapter> {
    fn default() -> Self {
        EasyHttpMock::new(EasyHttpMockConfig::default())
    }
}

impl ServerAdapter for VetisServerAdapter {
    type Config = ServerConfig;

    fn new(config: Self::Config) -> Self {
        Self {
            server: Vetis::new(config),
        }
    }

    fn base_url(&self) -> String {
        self.server.config().gen_url()
    }

    async fn start<H, Fut>(&mut self, handler: H) -> Result<(), EasyHttpMockError>
    where
        H: Fn(RequestType) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<ResponseType, VetisError>> + Send + 'static,
    {
        self.server
            .start(handler)
            .await
            .map_err(|e| EasyHttpMockError::Server(ServerError::Start(e.to_string())))
    }

    async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        self.server
            .stop()
            .await
            .map_err(|e| EasyHttpMockError::Server(ServerError::Stop(e.to_string())))
    }
}
