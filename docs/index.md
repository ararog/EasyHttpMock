---
layout: default
title: EasyHttpMock - The effortless HTTP mock server
nav_order: 1
description: "🧪 The effortless HTTP mock server for seamless API testing"
permalink: /
---
<div align="center">
<h1><b>EasyHttpMock</b></h1>
</div>

[![crates.io](https://img.shields.io/crates/v/easyhttpmock?style=flat-square)](https://crates.io/crates/easyhttpmock) 
[![Build Status](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml) 
[![Documentation](https://docs.rs/easyhttpmock/badge.svg)](https://docs.rs/easyhttpmock/latest/easyhttpmock)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**EasyHttpMock** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. Built on top of [VeTiS](https://github.com/ararog/vetis), it provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable.

## Features

- **🎯 Testing-Focused**: Purpose-built for HTTP client testing scenarios
- **⚡ Lightning Fast**: Powered by VeTiS for optimal performance
- **🔧 Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **🌐 Full Protocol Support**: HTTP/1, HTTP/2, and HTTP/3 compatibility
- **🛡️ Secure Testing**: Built-in TLS support for HTTPS endpoint testing
- **📦 Minimal Dependencies**: Lightweight footprint for your test suite

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
easyhttpmock = { version = "0.0.9" }
```

Basic usage:

```rust
use deboa::{Client, request::get, Result};
use deboa_extras::http::serde::json::JsonBody;
use serde::Deserialize;

#[derive(Deserialize)]
struct Post {
    id: u64,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    
    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")
        .send_with(&client)
        .await?
        .body_as(JsonBody, Post)?;
    
    println!("First post: {}", posts[0].title);
    Ok(())
}
```

## Examples

Check out the [examples](./examples.md) for complete examples of how to use EasyHttpMock in your projects.

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/easyhttpmock-templates`

## Documentation

- [API Reference](https://docs.rs/easyhttpmock)
- [Migration Guide](./MIGRATION_GUIDE.md)
- [Contributing Guide](./CONTRIBUTING.md)

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
