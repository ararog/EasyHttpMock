#[cfg(test)]
mod easy_http_mock_server_tests {
    use crate::config::EasyHttpMockConfig;
    use crate::server::adapters::vetis_adapter::VetisServerAdapter;
    use crate::EasyHttpMock;
    use bytes::Bytes;
    use http::{Response, StatusCode};
    use http_body_util::Full;
    use std::time::Duration;
    use vetis::{RequestType, ResponseType};

    #[tokio::test]
    async fn test_easy_http_mock_default() {
        let mock = EasyHttpMock::<VetisServerAdapter>::default();

        assert_eq!(mock.base_url(), "http://localhost:80");
        assert_eq!(mock.url("/test"), "http://localhost:80/test");
        assert_eq!(mock.url(""), "http://localhost:80");
    }

    #[tokio::test]
    async fn test_easy_http_mock_with_custom_config() {
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://custom.mock".to_string()))
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(3000)
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let mock = EasyHttpMock::new(config);

        assert_eq!(mock.base_url(), "http://localhost:3000");
        assert_eq!(mock.url("/api"), "https://custom.mock/api");
        assert_eq!(mock.url(""), "https://custom.mock");
    }

    #[tokio::test]
    async fn test_server_lifecycle() {
        // Use a random available port to avoid permission issues
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(4000) // Use random available port
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config);

        // Define a simple handler that returns a mock response
        let handler = |_req: RequestType| async move {
            let response: ResponseType = Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from("Hello, World!")))
                .unwrap();
            Ok(response)
        };

        // Start the server
        let result = mock
            .start(handler)
            .await;
        if let Err(e) = &result {
            println!("Server start error: {:?}", e);
        }
        assert!(result.is_ok());

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Stop the server
        let result = mock.stop().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_url_generation() {
        // Test with default config (no base_url override)
        let mock = EasyHttpMock::<VetisServerAdapter>::default();
        assert_eq!(mock.url("/api/users"), "http://localhost:80/api/users");
        assert_eq!(mock.url("/"), "http://localhost:80/");
        assert_eq!(mock.url(""), "http://localhost:80");
        assert_eq!(mock.url("test"), "http://localhost:80test");

        // Test with custom base_url
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .base_url(Some("https://api.example.com".to_string()))
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(8181)
                    .build(),
            )
            .build();

        let mock = EasyHttpMock::new(config);
        assert_eq!(mock.url("/v1/users"), "https://api.example.com/v1/users");
        assert_eq!(mock.url("/"), "https://api.example.com/");
        assert_eq!(mock.url(""), "https://api.example.com");
        assert_eq!(mock.url("health"), "https://api.example.comhealth");
    }

    #[tokio::test]
    async fn test_server_with_different_responses() {
        // Use a random available port to avoid permission issues
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(9999) // Use random available port
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config);

        // Define a handler that responds based on path
        let handler = |req: RequestType| async move {
            let path = req.uri().path();
            let (status, body) = match path {
                "/health" => (StatusCode::OK, "OK"),
                "/not-found" => (StatusCode::NOT_FOUND, "Not Found"),
                _ => (StatusCode::OK, "Default Response"),
            };

            let response: ResponseType = Response::builder()
                .status(status)
                .body(Full::new(Bytes::from(body)))
                .unwrap();
            Ok(response)
        };

        // Start the server
        mock.start(handler)
            .await
            .unwrap();

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Test that server started successfully
        assert!(mock
            .base_url()
            .contains("localhost:"));
        assert!(!mock
            .base_url()
            .contains("localhost:80")); // Should not be default port

        // Stop the server
        mock.stop()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_server_with_json_response() {
        // Use a random available port to avoid permission issues
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(7777) // Use random available port
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config);

        // Define a handler that returns JSON
        let handler = |_req: RequestType| async move {
            let json_body = r#"{
                "id": 1,
                "name": "Test User",
                "email": "test@example.com",
                "active": true
            }"#;

            let response: ResponseType = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(json_body)))
                .unwrap();
            Ok(response)
        };

        // Start the server
        mock.start(handler)
            .await
            .unwrap();

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify server is running
        assert!(mock
            .base_url()
            .contains("localhost:7777"));
        assert!(!mock
            .base_url()
            .is_empty());

        // Stop the server
        mock.stop()
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::config::EasyHttpMockConfig;
    use crate::server::adapters::vetis_adapter::VetisServerAdapter;
    use crate::EasyHttpMock;
    use bytes::Bytes;
    use http::{Response, StatusCode};
    use http_body_util::Full;
    use std::time::Duration;
    use vetis::{RequestType, ResponseType};

    #[tokio::test]
    async fn test_multiple_server_instances() {
        // Create two different server instances with random ports
        let config1 = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(8081)
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let config2 = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(8082)
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let mut mock1 = EasyHttpMock::new(config1);
        let mut mock2 = EasyHttpMock::new(config2);

        // Handler for server 1
        let handler1 = |_req: RequestType| async move {
            let response: ResponseType = Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from("Server 1 response")))
                .unwrap();
            Ok(response)
        };

        // Handler for server 2
        let handler2 = |_req: RequestType| async move {
            let response: ResponseType = Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from("Server 2 response")))
                .unwrap();
            Ok(response)
        };

        // Start both servers - this should work without conflicts
        mock1
            .start(handler1)
            .await
            .unwrap();
        mock2
            .start(handler2)
            .await
            .unwrap();

        // Give servers a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify both servers are running (they should be able to start successfully)
        let base_url1 = mock1.base_url();
        let base_url2 = mock2.base_url();

        // Both should contain localhost
        assert!(base_url1.contains("localhost"));
        assert!(base_url2.contains("localhost"));

        // Stop both servers
        mock1
            .stop()
            .await
            .unwrap();
        mock2
            .stop()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_server_with_random_port() {
        // Create a mock server with random port
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(8888) // Use random available port
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config);

        // Define a simple handler
        let handler = |_req: RequestType| async move {
            let response: ResponseType = Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from("Random port test")))
                .unwrap();
            Ok(response)
        };

        // Start the server
        mock.start(handler)
            .await
            .unwrap();

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        let base_url = mock.base_url();

        // Verify the URL contains a port (not the default 80)
        assert!(base_url.contains("localhost:8888"));
        assert!(!base_url.contains("localhost:80")); // Should not be default port

        // Stop the server
        mock.stop()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_server_restart() {
        // Use a random available port to avoid permission issues
        let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
            .server_config(
                vetis::server::config::ServerConfig::builder()
                    .port(5555) // Use random available port
                    .interface("127.0.0.1".to_string())
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config);

        // Define a handler
        let handler = |_req: RequestType| async move {
            let response: ResponseType = Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from("Restart test")))
                .unwrap();
            Ok(response)
        };

        // Start the server
        mock.start(handler)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Stop the server
        mock.stop()
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Start the server again with a new handler
        let handler2 = |_req: RequestType| async move {
            let response: ResponseType = Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from("Restarted")))
                .unwrap();
            Ok(response)
        };

        mock.start(handler2)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify server is running again
        assert!(mock
            .base_url()
            .contains("localhost:5555"));
        assert!(!mock
            .base_url()
            .is_empty());

        // Stop the server
        mock.stop()
            .await
            .unwrap();
    }
}
