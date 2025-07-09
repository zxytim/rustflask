use rust_flask::{FlaskApp, FlaskConfig, text_response, json_response, Json};
use hyper::{Method, Response, Body, StatusCode};
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
        port: 8080,
    };

    let app = FlaskApp::with_config(config);

    // Method-specific helpers
    app.get("/hello", |_req, _params| {
        text_response("Hello from GET method!")
    }).await;

    app.post("/users", |_req, _params| {
        // For now, return a simple response - async JSON parsing would require
        // more complex architecture with async trait support
        text_response("User creation endpoint (JSON parsing coming soon!)")
    }).await;

    // URL parameters
    app.get("/users/{id}/posts/{post_id}", |_req, params| {
        let user_id = params.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        let post_id = params.params.get("post_id").map(|s| s.as_str()).unwrap_or("unknown");
        text_response(&format!("User {} has post {}", user_id, post_id))
    }).await;

    // Configuration access
    let debug_mode = app.config().debug;
    println!("Debug mode: {}", debug_mode);

    app.run([127, 0, 0, 1], 8080).await;
}