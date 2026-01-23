# EasyHttpMock

🧪 **The effortless HTTP mock server for seamless API testing**

**EasyHttpMock** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. Built on top of [VeTiS](https://github.com/ararog/vetis), it provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable.

## ✨ Why EasyHttpMock?

- **🎯 Testing-Focused**: Purpose-built for HTTP client testing scenarios
- **⚡ Lightning Fast**: Powered by VeTiS for optimal performance
- **🔧 Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **🌐 Full Protocol Support**: HTTP/1, HTTP/2, and HTTP/3 compatibility
- **🛡️ Secure Testing**: Built-in TLS support for HTTPS endpoint testing
- **📦 Minimal Dependencies**: Lightweight footprint for your test suite

## 🛠️ Quick Start

Add EasyHttpMock to your `Cargo.toml`:

```rust
easyhttpmock = { version = "0.1.1", features = ["tokio-rt", "http2", "tokio-rust-tls"] }
```

## 💡 Usage Example

Here's how simple it is to create a mock HTTP server for testing:

```rust
use easyhttpmock::EasyHttpMock;
use http::{Request, Response};
use http_body_util::Full;
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock server
    let mock_server = EasyHttpMock::new()
        .port(8080)
        .mock_endpoint("/api/users", |req| {
            // Mock response for user endpoint
            Response::builder()
                .status(200)
                .body(Full::new(Bytes::from(r#"{"users": [{"id": 1, "name": "Alice"}]}"#)))
                .unwrap()
        })
        .start()
        .await?;

    // Your HTTP client can now test against the mock endpoint
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8080/api/users").send().await?;
    
    println!("Mock response: {}", response.text().await?);

    mock_server.stop().await?;
    Ok(())
}
```

## 🎯 Perfect For

- **🧪 Unit Testing**: Mock external APIs in your test suite
- **🔄 Integration Testing**: Test HTTP client behavior without real services
- **📊 Load Testing**: Simulate API responses under various conditions
- **🐛 Debugging**: Reproduce API issues in a controlled environment
- **📚 Documentation**: Create interactive API examples

## ⚙️ Supported Runtimes

- [tokio](https://github.com/tokio-rs/tokio) - High-performance async runtime
- [smol](https://github.com/smol-rs/smol) - Lightweight async runtime

## 🔧 Crate Features

- **tokio-rt** (default) - Tokio runtime support
- **smol-rt** - Smol runtime support  
- **http1** - HTTP/1 protocol support
- **http2** (default) - HTTP/2 protocol support
- **http3** - HTTP/3 protocol support
- **tokio-rust-tls** (default) - TLS support for Tokio
- **smol-rust-tls** - TLS support for Smol

## 📄 License

MIT

## 👤 Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>