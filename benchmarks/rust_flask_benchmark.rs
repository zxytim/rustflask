// RustFlask benchmark application to compare against Flask
use rust_flask::{FlaskApp, FlaskConfig, text_response, json_response};
use hyper::{Response, Body};
use serde_json;

#[tokio::main]
async fn main() {
    let config = FlaskConfig {
        debug: true,
        host: "127.0.0.1".to_string(),
        port: 8086,
    };

    let app = FlaskApp::with_config(config.clone());

    // Home page - HTML response
    app.get("/", |_req, _params| {
        Response::builder()
            .status(200)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(Body::from(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>RustFlask Benchmark Server</title>
</head>
<body>
    <h1>RustFlask Benchmark Server</h1>
    <p>This is the RustFlask server for benchmarking against Python Flask</p>
</body>
</html>
            "#))
            .unwrap()
    }).await;

    // Simple text greeting
    app.get("/hello", |_req, _params| {
        Response::builder()
            .status(200)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from("Hello from RustFlask! 🚀"))
            .unwrap()
    }).await;

    // JSON response template
    app.get("/hello/{name}", |_req, params| {
        let name = params.params.get("name").map(|s| s.as_str()).unwrap_or("World");
        Response::builder()
            .status(200)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from(format!("Hello, {}! Welcome to RustFlask! 🦀\n\n你好世界 (Hello World in Chinese)", name)))
            .unwrap()
    }).await;

    // JSON response
    app.get("/json", |_req, _params| {
        json_response(&serde_json::json!({
            "framework": "RustFlask",
            "version": "0.1.0",
            "features": {
                "http_methods": true,
                "url_parameters": true,
                "json_serialization": true,
                "configuration": true,
                "async_await": true,
                "utf8_encoding": true
            },
            "performance": "async",
            "encoding": "UTF-8",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "success": true,
            "programming_language": "Rust",
            "web_server": "Hyper",
            "runtime": "Tokio",
            "memory_safe": true,
            "type_safe": true
        }))
    }).await;

    // Multiple URL parameters
    app.get("/users/{id}/posts/{post_id}", |_req, params| {
        let user_id = params.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        let post_id = params.params.get("post_id").map(|s| s.as_str()).unwrap_or("unknown");
        
        json_response(&serde_json::json!({
            "message": "Multiple URL parameters captured successfully!",
            "user_id": user_id,
            "post_id": post_id,
            "path_info": format!("User {} -> Post {}", user_id, post_id),
            "parameters": {
                "id": user_id,
                "post_id": post_id
            },
            "demonstration": "url_parameter_extraction",
            "framework": "RustFlask",
            "encoding": "UTF-8"
        }))
    }).await;

    // POST endpoint
    app.post("/echo", |_req, _params| {
        Response::builder()
            .status(200)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from("POST request received successfully! 🎉\n\n✅ RustFlask:
   • HTTP method routing: ✅
   • Request handling: ✅
   • UTF-8 text responses: ✅
   • Memory safety: ✅
   • Type safety: ✅
   • Async performance: ✅"))
            .unwrap()
    }).await;

    // Health check endpoint
    app.get("/status", |_req, _params| {
        json_response(&serde_json::json!({
            "status": "running",
            "framework": "RustFlask",
            "version": "0.1.0",
            "technology_stack": {
                "language": "Rust",
                "http_server": "Hyper",
                "runtime": "Tokio",
                "json_library": "Serde",
                "charset": "UTF-8"
            },
            "features": {
                "http_methods": "✅",
                "url_parameters": "✅",
                "json_serialization": "✅", 
                "configuration": "✅",
                "async_await": "✅",
                "utf8_encoding": "✅",
                "memory_safety": "✅",
                "type_safety": "✅",
                "performance": "✅"
            },
            "encoding": "UTF-8",
            "performance": "async",
            "runtime": "memory_safe",
            "test_endpoint": true
        }))
    }).await;

    // Alive endpoint
    app.get("/alive", |_req, _params| {
        json_response(&serde_json::json!({
            "status": "alive",
            "message": "🟢 RustFlask server is running!",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "framework": "RustFlask", 
            "version": "0.1.0",
            "uptime": "running",
            "features": ["async", "methods", "parameters", "json", "utf8"],
            "performance": "optimal",
            "memory_safe": true,
            "type_safe": true
        }))
    }).await;

    println!("🚀 RustFlask Benchmark Server Started!");
    println!("👑 Running on: http://127.0.0.1:8086/");
    println!("📊 Benchmark ready - compare against Python Flask on port 8000");
    println!("🔧 All endpoints mirror Flask benchmark server");
    println!("");
    println!("💡 Endpoints to test:");
    println!("   • /hello           - Simple greeting");
    println!("   • /hello/World     - Personalized greeting");  
    println!("   • /json            - JSON response");
    println!("   • /users/123/posts/456 - Multi-params");
    println!("   • /echo (POST)     - POST method test");
    println!("   • /status          - Server status");
    println!("   • /alive           - Health check");

    // Start the server
    app.run([127, 0, 0, 1], config.port).await;
}