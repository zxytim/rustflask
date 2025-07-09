use rust_flask::{FlaskApp, FlaskConfig, text_response, json_response};
use hyper::{Method, Response, Body};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = FlaskConfig {
        debug: true,
        host: "127.0.0.1".to_string(),
        port: 8083,
    };

    let app = FlaskApp::with_config(config.clone());

    // Simple but beautiful HTML homepage with UTF-8 encoding
    app.get("/", |_req, _params| {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🚀 RustFlask Showcase</title>
    <style>
        body {
            font-family: 'Arial', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            margin: 0;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .container {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 20px;
            padding: 40px;
            max-width: 800px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.2);
        }
        h1 {
            text-align: center;
            font-size: 3em;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        .subtitle {
            text-align: center;
            font-size: 1.2em;
            margin-bottom: 30px;
            opacity: 0.9;
        }
        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin: 30px 0;
        }
        .feature {
            background: rgba(255, 255, 255, 0.1);
            padding: 20px;
            border-radius: 10px;
            border-left: 4px solid #4ecdc4;
        }
        .endpoints {
            background: rgba(0, 0, 0, 0.2);
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
        }
        .endpoint {
            background: rgba(255, 255, 255, 0.1);
            padding: 10px;
            margin: 10px 0;
            border-radius: 5px;
            display: flex;
            align-items: center;
        }
        .method {
            background: #4ecdc4;
            color: white;
            padding: 2px 6px;
            border-radius: 3px;
            font-size: 0.8em;
            margin-right: 10px;
            font-weight: bold;
        }
        .method.post { background: #ff6b6b; }
        .url {
            color: #4ecdc4;
            text-decoration: none;
            font-weight: bold;
        }
        .url:hover { text-decoration: underline; }
        .instructions {
            background: rgba(0, 0, 0, 0.2);
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
        }
        .footer {
            text-align: center;
            margin-top: 30px;
            opacity: 0.9;
        }
        .badge {
            display: inline-block;
            background: rgba(255, 255, 255, 0.2);
            padding: 5px 10px;
            border-radius: 15px;
            margin: 5px;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 RustFlask Showcase</h1>
        <div class="subtitle">✨ A Modern Async Web Framework for Rust ✨</div>

        <div class="features">
            <div class="feature">
                <h3>🌍 HTTP Method Helpers</h3>
                <p>Clean API with <code>get()</code> and <code>post()</code> methods for routing</p>
            </div>
            <div class="feature">
                <h3>🌐 URL Parameters</h3>
                <p>Dynamic routes with <code>{param}</code> syntax</p>
            </div>
            <div class="feature">
                <h3>📊 JSON Support</h3>
                <p>Built-in serialization with Serde</p>
            </div>
            <div class="feature">
                <h3>⚡ Async/Await</h3>
                <p>Built on Tokio for maximum performance</p>
            </div>
            <div class="feature">
                <h3>🔧 Configuration</h3>
                <p>Debug mode and custom settings</p>
            </div>
            <div class="feature">
                <h3>🛡️ Type Safety</h3>
                <p>Memory safety with Rust's type system</p>
            </div>
        </div>

        <div class="endpoints">
            <h2>🔗 Available Endpoints</h2>
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/hello" class="url">/hello</a>
                <span style="margin-left: 10px;">- Simple greeting</span>
            </div>
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/hello/World" class="url">/hello/{name}</a>
                <span style="margin-left: 10px;">- Personalized greeting</span>
            </div>
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/json" class="url">/json</a>
                <span style="margin-left: 10px;">- JSON response example</span>
            </div>
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/users/123/posts/456" class="url">/users/{id}/posts/{post_id}</a>
                <span style="margin-left: 10px;">- Multiple parameters</span>
            </div>
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/alive" class="url">/alive</a>
                <span style="margin-left: 10px;">- Health check</span>
            </div>
            <div class="endpoint">
                <span class="method">POST</span>
                <a href="/echo" class="url">/echo</a>
                <span style="margin-left: 10px;">- POST method demo</span>
            </div>
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/api" class="url">/api</a>
                <span style="margin-left: 10px;">- API documentation</span>
            </div>
        </div>

        <div class="instructions">
            <h2>🚀 Try It Now!</h2>
            <p>Click on any of the URLs above, or try these cURL commands:</p>
            <div style="background: rgba(0,0,0,0.1); padding: 15px; border-radius: 8px; margin: 10px 0;">
                <strong>Personalized Greeting:</strong><br>
                <code>curl http://localhost:8083/hello/Rustacean</code>
            </div>
            <div style="background: rgba(0,0,0,0.1); padding: 15px; border-radius: 8px; margin: 10px 0;">
                <strong>Multiple Parameters:</strong><br> 
                <code>curl http://localhost:8083/users/42/posts/1337</code>
            </div>
            <div style="background: rgba(0,0,0,0.1); padding: 15px; border-radius: 8px; margin: 10px 0;">
                <strong>POST Request:</strong><br>
                <code>curl -X POST http://localhost:8083/echo</code>
            </div>
        </div>

        <div style="text-align: center; margin: 30px 0;">
            <span class="badge">🦀 Rust Powered</span>
            <span class="badge">⚡ Async/ Await</span>
            <span class="badge">🌍 UTF-8</span>
            <span class="badge">🔒 Memory Safety</span>
            <span class="badge">🎮 Interactive</span>
        </div>

        <div class="footer">
            <p>🦀 Powered by Rust • Built with ❤️ for the community</p>
            <p>⚡ Experience the power of async web development in Rust</p>
        </div>
    </div>
</body>
</html>"#;

        Response::builder()
            .status(200)
            .header("Content-Type", "text/html; charset=utf-8")
            .header("Content-Length", html.len().to_string())
            .body(Body::from(html))
            .unwrap()
    }).await;
    
    // Simple greeting with UTF-8 encoding
    app.get("/hello", |_req, _params| {
        Response::builder()
            .status(200)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from("👋 Hello from RustFlask! 🦀\n\n✨ This demonstrates UTF-8 encoding with emojis!"))
            .unwrap()
    }).await;

    // API documentation endpoint
    app.get("/api", |_req, _params| {
        let api_docs = serde_json::json!({
            "framework": "RustFlask",
            "version": "0.1.0", 
            "description": "A modern async web framework for Rust",
            "endpoints": [
                {
                    "method": "GET",
                    "path": "/",
                    "description": "Beautiful HTML homepage with UTF-8 encoding"
                },
                {
                    "method": "GET",
                    "path": "/hello",
                    "description": "Simple greeting message"
                },
                {
                    "method": "GET", 
                    "path": "/hello/{name}",
                    "description": "Personalized greeting with URL parameter"
                },
                {
                    "method": "GET",
                    "path": "/json",
                    "description": "JSON response example"
                },
                {
                    "method": "GET",
                    "path": "/users/{id}/posts/{post_id}",
                    "description": "Multiple URL parameters demo"
                },
                {
                    "method": "GET",
                    "path": "/alive", 
                    "description": "Health check endpoint"
                },
                {
                    "method": "POST",
                    "path": "/echo",
                    "description": "POST method demonstration"
                },
                {
                    "method": "GET",
                    "path": "/api",
                    "description": "This API documentation"
                }
            ],
            "features": [
                "HTTP Method Helpers",
                "URL Parameters",
                "JSON Serialization",
                "Configuration Management", 
                "Async/Await Performance",
                "UTF-8 HTML Responses",
                "Type Safety",
                "Memory Safety"
            ]
        });
        json_response(&api_docs)
    }).await;

    // Simple greeting
    app.get("/hello", |_req, _params| {
        text_response("👋 Hello from RustFlask! 🦀\n\n✨ This demonstrates GET method handling with:
   • UTF-8 encoding ✅
   • Emoji support ✅
   • Simple text responses ✅")
    }).await;

    // Personalized greeting with URL parameter  
    app.get("/hello/{name}", |_req, params| {
        let name = params.params.get("name").map(|s| s.as_str()).unwrap_or("World");
        Response::builder()
            .status(200)
            .header("Content-Type", "text/plain; charset=utf-8") 
            .body(Body::from(format!("🎉 Hello, {}! 🌟\n\n🚀 Welcome to RustFlask!\n\n🔧 URL parameter received: name={}", 
                name, name)))
            .unwrap()
    }).await;

    // JSON response with comprehensive data
    app.get("/json", |_req, _params| {
        let data = serde_json::json!({
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
            "adj": "excellent",
            "rust_features": ["memory_safety", "zero_cost_abstractions", "fearless_concurrency"],
            "example": {
                "GET": "/hello/{name}",
                "POST": "/echo",
                "params": ["{id}", "{post_id}"],
                "features": "fully_demosntrated"
            }
        });
        json_response(&data)
    }).await;

    // Complex URL parameters with multiple parameters
    app.get("/users/{id}/posts/{post_id}", |_req, params| {
        let user_id = params.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        let post_id = params.params.get("post_id").map(|s| s.as_str()).unwrap_or("unknown");
        
        let response = serde_json::json!({
            "message": "🎯 Multiple URL parameters captured successfully!",
            "user_id": user_id,
            "post_id": post_id,
            "path_info": format!("User {} -> Post {}", user_id, post_id),
            "parameters": {
                "id": user_id,
                "post_id": post_id
            },
            "demonstration": {
                "url_format": "/users/{id}/posts/{post_id}",
                "current_values": [
                    user_id,
                    post_id
                ]
            },
            "features": "url_parameter_extraction"
        });
        json_response(&response)
    }).await;

    // POST endpoint demonstration
    app.post("/echo", |_req, _params| {
        text_response("📬 POST request received successfully! 🎊\n\n✅ This demonstrates POST method handling in RustFlask:\n   • HTTP method routing ✅\n   • Request handling ✅\n   • UTF-8 text responses ✅\n   • Framework versatility ✅")
    }).await;

    // Health check endpoint
    app.get("/alive", |_req, _params| {
        let now = chrono::Utc::now();
        json_response(&serde_json::json!({
            "status": "alive",
            "message": "🟢 All systems operational!",
            "timestamp": now.to_rfc3339(),
            "framework": "RustFlask",
            "version": "0.1.0",
            "uptime": "running",
            "features": ["async", "methods", "parameters", "json", "utf8"],
            "health": "excellent",
            "encoding": "UTF-8",
            "performance": "optimal"
        }))
    }).await;

    // Catch-all for undefined routes with helpful suggestions
    app.get("/*", |_req, params| {
        let path = params.params.get("*").map(|s| s.as_str()).unwrap_or("unknown");
        let help_text = format!(r#"🔍 Route Not Found: {}

💡 Available Routes:
  • GET /         - Beautiful homepage
  • GET /hello    - Simple greeting
  • GET /hello/{{name}} - Personalized greeting
  • GET /json     - JSON response
  • GET /users/{{id}}/posts/{{post_id}} - URL parameters
  • POST /echo    - POST method demo
  • GET /alive    - Health check
  • GET /api      - API documentation

🔧 Framework Features:
  ✅ HTTP Method Helpers
  ✅ URL Parameters
  ✅ JSON Processing  
  ✅ Configuration
  ✅ Async/Await
  ✅ UTF-8 Encoding

🔗 Try one of these endpoints instead!"#, path);
        
        Response::builder()
            .status(404)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from(help_text))
            .unwrap()
    }).await;

    // Print startup banner
    println!("\n🚀🦀 RUSTFLASK SHOWCASE 🦀🚀");
    println!("================================================================");
    println!("🌐 Server running on: http://{}/    📱 Open your browser!", config.host);
    println!("🔧 Port: {}    🐛 Debug mode: {}", config.port, config.debug);
    println!("================================================================");
    println!("💡 QUICK START:");
    println!("   • http://localhost:{}/            - Homepage with demo", config.port);
    println!("   • http://localhost:{}/hello/Rust  - Personalized greeting", config.port);
    println!("   • http://localhost:{}/json        - JSON response", config.port);
    println!("   • http://localhost:{}/alive       - Health check", config.port);
    println!("   • curl -X POST http://localhost:{}/echo", config.port);
    println!("🎯 FEATURES DEMONSTRATED:");
    println!("   • HTTP Methods (GET & POST)");
    println!("   • URL Parameters ({{id}}, {{post_id}}, {{name}})");
    println!("   • UTF-8 HTML & text responses");
    println!("   • JSON serialization");
    println!("   • Configuration management");
    println!("   • Error handling with friendly messages");
    println!("   • Type safety & memory safety");
    println!("================================================================
");

    // Start the server
    app.run([127, 0, 0, 1], config.port).await;
}