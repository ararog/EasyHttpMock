#[cfg(test)]
mod easy_http_mock_config_tests {
    use crate::config::EasyHttpMockConfig;
    use crate::server::adapters::vetis_adapter::VetisServerAdapter;
    use vetis::server::config::{SecurityConfig, ServerConfig};

    #[test]
    fn test_easy_http_mock_config_default() {
        let config = EasyHttpMockConfig::<VetisServerAdapter>::default();

        assert!(config
            .base_url()
            .is_some());
        assert_eq!(
            config
                .base_url()
                .as_ref()
                .unwrap(),
            "http://localhost:80"
        );
        assert_eq!(
            config
                .server_config()
                .port(),
            80
        );
        assert_eq!(
            config
                .server_config()
                .interface(),
            "0.0.0.0"
        );
        assert!(config
            .server_config()
            .security()
            .is_none());
    }

    #[test]
    fn test_easy_http_mock_config_builder_default() {
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder().build();

        assert!(config
            .base_url()
            .is_none());
        assert_eq!(
            config
                .server_config()
                .port(),
            80
        ); // Default from ServerConfigBuilder
        assert_eq!(
            config
                .server_config()
                .interface(),
            "0.0.0.0"
        );
        assert!(config
            .server_config()
            .security()
            .is_none());
    }

    #[test]
    fn test_easy_http_mock_config_builder_with_base_url() {
        let base_url = "https://api.example.com".to_string();
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some(base_url.clone()))
            .build();

        assert_eq!(config.base_url(), &Some(base_url));
        assert_eq!(
            config
                .server_config()
                .port(),
            80
        );
        assert_eq!(
            config
                .server_config()
                .interface(),
            "0.0.0.0"
        );
    }

    #[test]
    fn test_easy_http_mock_config_builder_with_none_base_url() {
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(None)
            .build();

        assert!(config
            .base_url()
            .is_none());
        assert_eq!(
            config
                .server_config()
                .port(),
            80
        );
    }

    #[test]
    fn test_easy_http_mock_config_builder_with_server_config() {
        let server_config = ServerConfig::builder()
            .port(3000)
            .interface("127.0.0.1".to_string())
            .build();

        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(server_config.clone())
            .build();

        assert!(config
            .base_url()
            .is_none());
        assert_eq!(
            config
                .server_config()
                .port(),
            3000
        );
        assert_eq!(
            config
                .server_config()
                .interface(),
            "127.0.0.1"
        );
    }

    #[test]
    fn test_easy_http_mock_config_builder_complete() {
        let base_url = "https://test.mock".to_string();
        let server_config = ServerConfig::builder()
            .port(8443)
            .interface("localhost".to_string())
            .security(
                SecurityConfig::builder()
                    .cert_from_bytes(vec![1, 2, 3])
                    .key_from_bytes(vec![4, 5, 6])
                    .client_auth(true)
                    .build(),
            )
            .build();

        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some(base_url.clone()))
            .server_config(server_config.clone())
            .build();

        assert_eq!(config.base_url(), &Some(base_url));
        assert_eq!(
            config
                .server_config()
                .port(),
            8443
        );
        assert_eq!(
            config
                .server_config()
                .interface(),
            "localhost"
        );
        assert!(config
            .server_config()
            .security()
            .is_some());
        assert!(config
            .server_config()
            .security()
            .unwrap()
            .client_auth());
    }

    #[test]
    fn test_easy_http_mock_config_builder_chaining() {
        let security_config = SecurityConfig::builder()
            .cert_from_bytes(vec![10, 20, 30])
            .build();

        let server_config = ServerConfig::builder()
            .port(9090)
            .interface("192.168.1.100".to_string())
            .security(security_config)
            .build();

        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://chained.mock".to_string()))
            .server_config(server_config)
            .build();

        assert_eq!(
            config
                .base_url()
                .as_ref()
                .unwrap(),
            "https://chained.mock"
        );
        assert_eq!(
            config
                .server_config()
                .port(),
            9090
        );
        assert_eq!(
            config
                .server_config()
                .interface(),
            "192.168.1.100"
        );
    }

    #[test]
    fn test_easy_http_mock_config_empty_base_url() {
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("".to_string()))
            .build();

        assert_eq!(config.base_url(), &Some("".to_string()));
    }

    #[test]
    fn test_easy_http_mock_config_long_base_url() {
        let long_url = "https://very.long.domain.name.with.many.subdomains.for.testing.purposes.example.com:8443/api/v1".to_string();
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some(long_url.clone()))
            .build();

        assert_eq!(config.base_url(), &Some(long_url));
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::config::EasyHttpMockConfig;
    use crate::server::adapters::vetis_adapter::VetisServerAdapter;
    use vetis::server::config::{SecurityConfig, ServerConfig};

    #[test]
    fn test_config_with_vetis_server_adapter_integration() {
        let security_config = SecurityConfig::builder()
            .cert_from_bytes(
                b"-----BEGIN CERTIFICATE-----\nMOCK CERT\n-----END CERTIFICATE-----".to_vec(),
            )
            .key_from_bytes(
                b"-----BEGIN PRIVATE KEY-----\nMOCK KEY\n-----END PRIVATE KEY-----".to_vec(),
            )
            .build();

        let server_config = ServerConfig::builder()
            .port(443)
            .interface("0.0.0.0".to_string())
            .security(security_config)
            .build();

        let mock_config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://secure.mock".to_string()))
            .server_config(server_config)
            .build();

        assert_eq!(mock_config.base_url(), &Some("https://secure.mock".to_string()));
        assert_eq!(
            mock_config
                .server_config()
                .port(),
            443
        );
        assert!(mock_config
            .server_config()
            .security()
            .is_some());
    }

    #[test]
    fn test_multiple_config_instances() {
        let config1 = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://config1.mock".to_string()))
            .server_config(
                ServerConfig::builder()
                    .port(8080)
                    .build(),
            )
            .build();

        let config2 = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://config2.mock".to_string()))
            .server_config(
                ServerConfig::builder()
                    .port(9090)
                    .build(),
            )
            .build();

        assert_ne!(config1.base_url(), config2.base_url());
        assert_ne!(
            config1
                .server_config()
                .port(),
            config2
                .server_config()
                .port()
        );
    }

    #[test]
    fn test_config_immutability() {
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://immutable.mock".to_string()))
            .server_config(
                ServerConfig::builder()
                    .port(3000)
                    .build(),
            )
            .build();

        // Original config should remain unchanged
        assert_eq!(config.base_url(), &Some("https://immutable.mock".to_string()));
        assert_eq!(
            config
                .server_config()
                .port(),
            3000
        );

        // Creating a new config shouldn't affect the original
        let _new_config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://new.mock".to_string()))
            .server_config(
                ServerConfig::builder()
                    .port(4000)
                    .build(),
            )
            .build();

        assert_eq!(config.base_url(), &Some("https://immutable.mock".to_string()));
        assert_eq!(
            config
                .server_config()
                .port(),
            3000
        );
    }

    #[test]
    fn test_config_with_different_server_configs() {
        let http_config = ServerConfig::builder()
            .port(80)
            .interface("0.0.0.0".to_string())
            .build();

        let https_config = ServerConfig::builder()
            .port(443)
            .interface("0.0.0.0".to_string())
            .security(
                SecurityConfig::builder()
                    .cert_from_bytes(vec![1, 2, 3])
                    .key_from_bytes(vec![4, 5, 6])
                    .build(),
            )
            .build();

        let http_mock = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("http://http.mock".to_string()))
            .server_config(http_config)
            .build();

        let https_mock = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://https.mock".to_string()))
            .server_config(https_config)
            .build();

        assert_eq!(http_mock.base_url(), &Some("http://http.mock".to_string()));
        assert_eq!(
            http_mock
                .server_config()
                .port(),
            80
        );
        assert!(http_mock
            .server_config()
            .security()
            .is_none());

        assert_eq!(https_mock.base_url(), &Some("https://https.mock".to_string()));
        assert_eq!(
            https_mock
                .server_config()
                .port(),
            443
        );
        assert!(https_mock
            .server_config()
            .security()
            .is_some());
    }
}
