use rust_flask::{FlaskApp, FlaskConfig, text_response, json_response};
use hyper::Method;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let config = FlaskConfig {
        debug: true,
        host: "127.0.0.1".to_string(),
        port: 8081,  // Changed port to avoid conflict
    };

    let app = FlaskApp::with_config(config.clone());

    // HOME PAGE - Shows framework overview
    app.get("/", |_req, _params| {
        let welcome = r#"
🚀 RustFlask Framework Showcase

📝 Welcome to the RustFlask demo!

🔗 Available Endpoints:
  • /hello - Simple GET response
  • /hello/{name} - URL parameter demo  
  • /json - JSON response demo
  • /users/{id}/posts/{post_id} - Multiple parameters
  • /info - Configuration info
  • /echo (POST) - POST method demo

🎯 Features Demonstrated:
  ✅ HTTP Method Helpers (get/post)
  ✅ URL Parameters ({id}, {post_id})
  ✅ Configuration Management
  ✅ Text & JSON Responses
  ✅ Async/Await Performance
  ✅ Debug Mode

Try them out with curl or your browser!
        "#;
        text_response(welcome.trim())
    }).await;

    // Simple GET endpoint
    app.get("/hello", |_req, _params| {
        text_response("👋 Hello from RustFlask! Try /hello/yourname too!")
    }).await;

    // URL parameter example
    app.get("/hello/{name}", |_req, params| {
        let name = params.params.get("name").map(|s| s.as_str()).unwrap_or("World");
        text_response(&format!("🎉 Hello, {}! Welcome to RustFlask!", name))
    }).await;

    // JSON response example
    app.get("/json", |_req, _params| {
        let data = serde_json::json!({
            "framework": "RustFlask",
            "version": "0.1.0",
            "features": ["async", "methods", "parameters", "json"],
            "success": true
        });
        json_response(&data)
    }).await;

    // Complex URL parameters example
    app.get("/users/{id}/posts/{post_id}", |_req, params| {
        let user_id = params.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        let post_id = params.params.get("post_id").map(|s| s.as_str()).unwrap_or("unknown");
        
        let response = serde_json::json!({
            "message": "URL parameters parsed successfully!",
            "user_id": user_id,
            "post_id": post_id,
            "note": "You accessed: /users/{}/posts/{}",
            "path_info": format!("User {} -> Post {}", user_id, post_id)
        });
        json_response(&response)
    }).await;

    // Configuration info endpoint
    app.get("/info", |_req, _params| {
        let version_info = r#"
📊 Framework Information

🔧 Technology Stack:
  • Language: Rust
  • Async Runtime: Tokio
  • HTTP Server: Hyper
  • JSON: Serde

📋 API Endpoints:
  • GET /hello - Simple greeting
  • GET /hello/{name} - Personalized greeting
  • GET /json - JSON response demo
  • GET /users/{id}/posts/{post_id} - Complex URL parameters
  • GET /info - This information
  • POST /echo - POST method demo

🛠️ Framework Features:
  • Async/Await Support
  • HTTP Method Routing
  • URL Parameter Extraction
  • JSON Serialization
  • Configuration Management
  • Error Handling
        "#;
        text_response(version_info.trim())
    }).await;

    // POST endpoint example
    app.post("/echo", |_req, _params| {
        text_response("📬 POST request received! This demonstrates the POST method helper.")
    }).await;

    // Debug endpoint - only shows config when debug mode is on
    app.get("/debug", |_req, _params| {
        if cfg!(debug_assertions) {
            text_response("🐛 Debug mode is ACTIVE! All internal features are enabled.")
        } else {
            text_response("🔒 Debug mode is disabled. Running in production mode.")
        }
    }).await;

    // Catch-all 404 handler
    app.get("/*", |_req, params| {
        let path = params.params.get("*").map(|s| s.as_str()).unwrap_or("unknown");
        let not_found = format!(
            r#"
🔍 Route Not Found: {}

💡 Available Routes:
  • GET /      - Home page
  • GET /hello - Simple greeting
  • GET /json  - JSON response

📝 Try visiting one of these endpoints instead!
            "#,
            path
        );
        text_response(&not_found.trim())
    }).await;

    // Print startup info
    println!("🚀 RustFlask Showcase
");
    println!("🌐 Server running on http://{}/", config.host);
    println!("📋 Visit the endpoints to test all features!");
    println!("💡 Try starting with http://{}/hello/yourname", config.host);

    app.run([127, 0, 0, 1], config.port).await;
}