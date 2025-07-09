# RustFlask

A simple web framework inspired by Flask, implemented in Rust.

## Features

- HTTP Method Helpers: `get()` and `post()` for clean method-specific routing
- URL Parameters: Dynamic routes with `{param}` syntax (e.g., `/users/{id}`)
- Request Parsing: JSON request body parsing with `Json<T>`
- Configuration Management: Debug mode, custom host/port settings
- Async/Await: Built on tokio and hyper for async runtime
- Text & JSON Responses: Built-in response helpers

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rust_flask = { path = "./rust_flask" }
```

## Example

```rust
use rust_flask::{FlaskApp, text_response, json_response};
use hyper::{Method, Response, Body, StatusCode};

#[tokio::main]
async fn main() {
    let app = FlaskApp::new();

    // Add routes
    app.route("/", |req, _params| {
        if req.method() == Method::GET {
            text_response("Hello, World!")
        } else {
            Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::from("Method not allowed"))
                .unwrap()
        }
    }).await;

    app.route("/json", |req, _params| {
        if req.method() == Method::GET {
            let data = vec!["hello", "world"];
            json_response(&data)
        } else {
            Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::from("Method not allowed"))
                .unwrap()
        }
    }).await;

    // Start server
    app.run([127, 0, 0, 1], 8080).await;
}
```

## Run Examples

### Modern Designs with UTF-8 Encoding
```bash
# Modern Tech Magazine Style (Recommended)
cargo run --example combined_demo      # http://localhost:8085/

# Executive Corporate Style (Professional)
cargo run --example fixed_beautiful_showcase  # http://localhost:8084/
```

### Basic Examples for Learning
```bash
# Simple UTF-8 encoding test
cargo run --example test_encoding      # http://localhost:8001/

# Basic feature demonstration
cargo run --example showcase           # http://localhost:8081/

# Enhanced functionality demo
cargo run --example enhanced_example   # http://localhost:8083/

# Interactive playground
cargo run --example fancy_showcase     # http://localhost:8082/
```

## Testing

```bash
cargo test
```

## API

### FlaskApp

- `new()` - Create a new app instance with default config
- `with_config(config)` - Create app with custom configuration
- `route(path, handler).await` - Register a route handler
- `get(path, handler).await` - Register GET route handler
- `post(path, handler).await` - Register POST route handler  
- `run(addr, port).await` - Start the server
- `config()` - Get reference to configuration

### FlaskConfig

- `debug: bool` - Enable debug mode
- `host: String` - Server host address  
- `port: u16` - Server port

### RouteParams

- `params: HashMap<String, String>` - Request path parameters
- `get(key)` - Get parameter value
- `insert(key, value)` - Insert parameter

### Helper Functions

- `text_response(content)` - Create a text/plain response
- `json_response(data)` - Create a JSON response

### Request Parsing

- `Json::<T>::from_request(req)` - Parse JSON request body

## URL Parameters

Use `{parameter}` syntax in route paths:
- `/users/{id}` - captures user ID
- `/posts/{post_id}/comments/{comment_id}` - multiple parameters
- Parameters are accessible via `params.get("param_name")`

## Features Added

1. **HTTP Method Helpers**: Dedicated `get()` and `post()` methods
2. **URL Parameters**: Dynamic route parameters with `{param}` syntax
3. **Configuration Management**: Debug mode and server settings
4. **JSON Request Parsing**: Type-safe JSON body parsing with serde
5. **Route Pattern Matching**: Support for method-specific routes
6. **Enhanced Error Handling**: Better 404 and 405 responses