use crate::server::ServerAdapter;

pub struct EasyHttpMockConfigBuilder<S>
where
    S: ServerAdapter,
{
    base_url: Option<String>,
    pub(crate) server_config: S::Config,
}

impl<S> EasyHttpMockConfigBuilder<S>
where
    S: ServerAdapter,
{
    pub fn base_url(mut self, base_url: Option<String>) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn server_config(mut self, server_config: S::Config) -> Self {
        self.server_config = server_config;
        self
    }

    pub fn build(self) -> EasyHttpMockConfig<S> {
        EasyHttpMockConfig {
            base_url: self.base_url,
            server_config: self.server_config,
        }
    }
}

#[derive(Clone)]
pub struct EasyHttpMockConfig<S>
where
    S: ServerAdapter,
{
    pub(crate) base_url: Option<String>,
    pub(crate) server_config: S::Config,
}

impl<S> Default for EasyHttpMockConfig<S>
where
    S: ServerAdapter + Default,
    S::Config: Clone + Default,
{
    fn default() -> Self {
        Self {
            base_url: None,
            server_config: S::Config::default(),
        }
    }
}

impl<S> EasyHttpMockConfig<S>
where
    S: ServerAdapter,
    S::Config: Clone + Default,
{
    pub fn builder() -> EasyHttpMockConfigBuilder<S> {
        EasyHttpMockConfigBuilder {
            base_url: None,
            server_config: S::Config::default(),
        }
    }

    pub fn base_url(&self) -> &Option<String> {
        &self.base_url
    }

    pub fn server_config(&self) -> &S::Config {
        &self.server_config
    }
}
