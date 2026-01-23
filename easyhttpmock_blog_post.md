# Simplify HTTP Testing with `easyhttpmock`

Testing HTTP requests and API integrations is an essential yet daunting part of modern software development. Whether you're working on a microservice architecture or developing an API-powered application, you need robust tools to ensure your application handles various scenarios with resilience. However, depending on live APIs and services can complicate testing workflows. Enter **[easyhttpmock](https://github.com/ararog/easyhttpmock)**, a lightweight yet powerful solution for mocking HTTP requests and responses.

This open-source library, written in **Rust**, is perfect for developers aiming to create isolated, repeatable, and consistent test environments while avoiding the pitfalls of live API dependencies. And the best part? It takes just a few lines of code to mock even complex HTTP use cases.

---

### Why `easyhttpmock`?

Here are a few reasons why `easyhttpmock` should be part of your testing toolkit:

- **Mocking Made Easy**: Define mock HTTP servers effortlessly and control what they return.
- **Error Injection**: Test your code’s resilience by simulating errors, timeouts, or specific HTTP status codes.
- **Flexibility**: It works seamlessly with any Rust-based testing framework and supports secure TLS/SSL configurations for end-to-end testing.
- **Open-source and Efficient**: Lightweight and community-collaborated, with the ability to inspect and even improve the codebase via its [GitHub repository](https://github.com/ararog/easyhttpmock).

---

### Real-world Example from `easyhttpmock`

The [examples directory](https://github.com/ararog/easyhttpmock/tree/main/examples) in the repository includes a simplified showcase of how to use `easyhttpmock`. Let’s break down an example from `examples/simple/src/main.rs`, a comprehensive demonstration of how to set up and use `easyhttpmock`.

```rust
use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use hyper::Response;

use easyhttpmock::{
    config::EasyHttpMockConfig,
    server::{adapters::vetis_adapter::VetisServerAdapter, PortGenerator},
    EasyHttpMock,
};

use deboa::{cert::ContentEncoding, request::DeboaRequest, Client};

use vetis::server::config::{SecurityConfig, ServerConfig};

// Including mock security certificate files.
pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuring security with certificates
    let tls_config = SecurityConfig::builder()
        .cert(SERVER_CERT.to_vec())
        .key(SERVER_KEY.to_vec())
        .build();

    // Building the configuration for the mock server
    let vetis_config = ServerConfig::builder()
        .security(tls_config)
        .with_random_port()
        .build();

    let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
        .server_config(vetis_config)
        .build();

    let mut server = EasyHttpMock::new(config);
    
    // Starting the mock server with a simple "Hello World" response
    let result = server
        .start(|_| async move {
            Ok(Response::new(Full::new(Bytes::from("Hello World"))))
        })
        .await;

    result.unwrap_or_else(|err| panic!("Failed to start mock server: {}", err));

    // Using a client to perform a GET request on the mock server
    let client = Client::builder()
        .certificate(deboa::cert::Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let request = DeboaRequest::get(server.url("/anything"))?.build()?
        .await?;

    // Validating the response
    if response.status() == StatusCode::OK {
        println!("Request executed successfully");
    }

    // Stopping the server after execution
    server.stop().await?;
    
    Ok(())
}
```

---

### Understanding the Code

#### Key Highlights:
1. **TLS/SSL Configuration**:
   The example uses mock certificates (`SERVER_CERT` and `SERVER_KEY`) to simulate a secure connection. These certificates are stored in the `examples/simple/certs` directory.

   ```rust
   let tls_config = SecurityConfig::builder()
       .cert(SERVER_CERT.to_vec())
       .key(SERVER_KEY.to_vec())
       .build();
   ```

2. **Mock Server Setup**:
   Using `VetisServerAdapter`, the example configures a mock server with a random port and sets it up to return a simple "Hello World" message for incoming requests.

   ```rust
   let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
       .server_config(vetis_config)
       .build();

   let result = server
       .start(|_| async move {
           Ok(Response::new(Full::new(Bytes::from("Hello World"))))
       })
       .await;
   ```

3. **Client Integration**:
   The code uses a `DeboaRequest` to send a GET request to the mocked URL and logs the result:

   ```rust
   let request = DeboaRequest::get(server.url("/anything"))?.build()?;
   let response = client.execute(request).await?;
   ```

4. **Server Shutdown**:
   The server is stopped gracefully at the end:

   ```rust
   server.stop().await?;
   ```

---

This example demonstrates how straightforward it is to set up a fully functional HTTP mocking server with `easyhttpmock`, integrate client requests, and handle SSL. Visit the [repository](https://github.com/ararog/easyhttpmock) to explore more examples and start building robust, isolated API tests today!