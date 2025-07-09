use rust_flask::{FlaskApp, FlaskConfig, text_response, json_response};
use hyper::{Response, Body};
use serde_json;

#[tokio::main]
async fn main() {
    let config = FlaskConfig {
        debug: true,
        host: "127.0.0.1".to_string(),
        port: 8085,
    };

    let app = FlaskApp::with_config(config.clone());

    // HOME PAGE - Combined demo with UTF-8 encoding
    app.get("/", |_req, _params| {
        let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🔧 RustFlask | Modern Async Framework</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --primary: #0f172a;
            --secondary: #1e293b;
            --accent: #3b82f6;
            --success: #10b981;
            --warning: #f59e0b;
            --error: #ef4444;
            --light: #f8fafc;
            --muted: #64748b;
            --shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
        }
        
        * { margin: 0; padding: 0; box-sizing: border-box; }
        
        body {
            font-family: 'Inter', system-ui, -apple-system, sans-serif;
            background: var(--light);
            color: var(--primary);
            line-height: 1.6;
            overflow-x: hidden;
        }
        
        .hero {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 80px 20px;
            text-align: center;
            position: relative;
            overflow: hidden;
        }
        
        .hero-content {
            position: relative;
            z-index: 1;
            max-width: 800px;
            margin: 0 auto;
        }
        
        .hero h1 {
            font-size: 3.5rem;
            font-weight: 700;
            margin-bottom: 1rem;
            text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }
        
        .hero .subtitle {
            font-size: 1.25rem;
            opacity: 0.9;
            font-weight: 300;
            margin-bottom: 2rem;
        }
        
        .hero .badge {
            display: inline-block;
            background: rgba(255, 255, 255, 0.2);
            padding: 8px 16px;
            border-radius: 20px;
            font-size: 0.875rem;
            margin: 4px;
            backdrop-filter: blur(10px);
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 80px 20px;
        }
        
        .section {
            margin-bottom: 80px;
        }
        
        .section-title {
            text-align: center;
            font-size: 2.5rem;
            font-weight: 600;
            margin-bottom: 3rem;
            color: var(--primary);
        }
        
        .features-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 30px;
            margin-bottom: 60px;
        }
        
        .feature-card {
            background: white;
            padding: 32px;
            border-radius: 16px;
            box-shadow: var(--shadow);
            transition: transform 0.2s ease;
            border: 1px solid #e2e8f0;
        }
        
        .feature-card:hover {
            transform: translateY(-2px);
        }
        
        .feature-card .icon {
            width: 48px;
            height: 48px;
            background: var(--accent);
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5rem;
            margin-bottom: 20px;
        }
        
        .feature-card h3 {
            font-size: 1.25rem;
            font-weight: 600;
            margin-bottom: 12px;
            color: var(--primary);
        }
        
        .feature-card p {
            color: var(--muted);
        }
        
        .endpoints-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 30px;
        }
        
        .endpoint-section {
            background: white;
            border-radius: 16px;
            padding: 32px;
            box-shadow: var(--shadow);
            border: 1px solid #e2e8f0;
        }
        
        .endpoint-section h3 {
            font-size: 1.5rem;
            font-weight: 600;
            margin-bottom: 24px;
            color: var(--primary);
            display: flex;
            align-items: center;
            gap: 12px;
        }
        
        .endpoint {
            background: #f1f5f9;
            padding: 16px;
            border-radius: 8px;
            margin-bottom: 12px;
            display: flex;
            align-items: center;
            gap: 12px;
        }
        
        .method {
            background: var(--accent);
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: 600;
            min-width: 40px;
            text-align: center;
        }
        
        .method.POST {
            background: var(--success);
        }
        
        .url {
            color: var(--accent);
            text-decoration: none;
            font-weight: 500;
            font-family: 'Courier New', monospace;
        }
        
        .url:hover {
            text-decoration: underline;
        }
        
        .cli-section {
            background: var(--secondary);
            border-radius: 16px;
            padding: 40px;
            margin: 60px 0;
            text-align: center;
        }
        
        .cli-section h3 {
            color: white;
            font-size: 1.5rem;
            font-weight: 600;
            margin-bottom: 20px;
        }
        
        .code {
            background: #0f172a;
            color: #34d399;
            padding: 24px;
            border-radius: 8px;
            font-family: 'Courier New', monospace;
            font-size: 0.9rem;
            line-height: 1.5;
            text-align: left;
            margin: 20px 0;
            border: 1px solid #334155;
            overflow-x: auto;
        }
        
        .footer {
            background: var(--primary);
            color: white;
            text-align: center;
            padding: 40px 20px;
            margin-top: 80px;
        }
        
        @media (max-width: 768px) {
            .hero h1 { font-size: 2.5rem; }
            .hero .subtitle { font-size: 1rem; }
            .section-title { font-size: 2rem; }
        }
    </style>
</head>
<body>
    <div class="hero">
        <div class="hero-content">
            <h1>🔧 RustFlask</h1>
            <div class="subtitle">Modern Async Web Framework Built in Rust</div>
            <div class="badges">
                <span class="badge">🦀 Rust Powered</span>
                <span class="badge">⚡ Async/Await</span>
                <span class="badge">🌐 UTF-8</span>
                <span class="badge">🔒 Memory Safe</span>
            </div>
        </div>
    </div>

    <div class="container">
        <div class="section">
            <h2 class="section-title">🚀 Why Choose RustFlask?</h2>
            <div class="features-grid">
                <div class="feature-card">
                    <div class="icon">⚡</div>
                    <h3>Blazing Fast Performance</h3>
                    <p>Built on Tokio and Hyper for maximum async performance with zero-cost abstractions.</p>
                </div>
                <div class="feature-card">
                    <div class="icon">🛡️</div>
                    <h3>Uncompromised Safety</h3>
                    <p>Memory safety and type safety with Rust's ownership system - no runtime exceptions.</p>
                </div>
                <div class="feature-card">
                    <div class="icon">🌐</div>
                    <h3>Modern Web Standards</h3>
                    <p>Full UTF-8 support, clean HTTP method routing, and proper response handling out of the box.</p>
                </div>
                <div class="feature-card">
                    <div class="icon">📊</div>
                    <h3>Type-Safe JSON APIs</h3>
                    <p>Built-in JSON serialization with Serde for reliable, type-safe API development.</p>
                </div>
            </div>
        </div>

        <div class="section">
            <h2 class="section-title">📡 Available Endpoints</h2>
            <div class="endpoints-grid">
                <div class="endpoint-section">
                    <h3>🌐 Web Interface</h3>
                    <div class="endpoint">
                        <span class="method">GET</span>
                        <a href="/utf8" class="url">/utf8</a>
                        <span>Unicode character test</span>
                    </div>
                    <div class="endpoint">
                        <span class="method">GET</span>
                        <a href="/status" class="url">/status</a>
                        <span>Server health status</span>
                    </div>
                </div>

                <div class="endpoint-section">
                    <h3>🔌 API Endpoints</h3>
                    <div class="endpoint">
                        <span class="method">GET</span>
                        <a href="/hello" class="url">/hello</a>
                        <span>JSON greeting</span>
                    </div>
                    <div class="endpoint">
                        <span class="method">GET</span>
                        <a href="/hello/:name" class="url">/hello/:name</a>
                        <span>Personalized greeting</span>
                    </div>
                    <div class="endpoint">
                        <span class="method">GET</span>
                        <a href="/json" class="url">/json</a>
                        <span>JSON demo response</span>
                    </div>
                    <div class="endpoint">
                        <span class="method">GET</span>
                        <a href="/users/:id/posts/:post_id" class="url">/users/:id/posts/:post_id</a>
                        <span>URL parameters demo</span>
                    </div>
                    <div class="endpoint">
                        <span class="method">POST</span>  
                        <a href="/echo" class="url">/echo</a>
                        <span>POST method demo</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="cli-section">
            <h3>💻 Command Line Examples</h3>
            <div class="code">
curl http://localhost:8085/hello/RustDeveloper
curl http://localhost:8085/users/42/posts/userguide
curl -X POST http://localhost:8085/echo

# Test UTF-8 support:
curl http://localhost:8085/hello/世界 | jq
            </div>
        </div>
    </div>

    <div class="footer">
        <h3>🦀 RustFlask v0.1.0</h3>
        <p>Built with ❤️ for the Rust community | Experience memory-safe async web development</p>
    </div>
</body>
</html>
        "#;

        Response::builder()
            .status(200)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(Body::from(html))
            .unwrap()
    }).await;

    // UTF-8 unicode test endpoint
    app.get("/utf8", |_req, _params| {
        let unicode_test = serde_json::json!({
            "success": true,
            "test": "unicode_characters",
            "emojis": ["🎉", "🚀", "🦀", "🌍", "✅", "🎯", "💻"],
            "accents": ["á", "é", "í", "ó", "ú", "Á", "É", "Í", "Ó", "Ú"],
            "languages": {
                "chinese": "你好世界",
                "japanese": "こんにちは",
                "korean": "안녕하세요",
                "arabic": "مرحبا بالعالم",
                "russian": "Привет мир",
                "greek": "Γεια σας κόσμε",
                "hebrew": "שלום עולם"
            },
            "mathematical": ["∑", "∫", "∂", "∇", "∆", "∞", "√", "π"],
            "encoding": "UTF-8",
            "description": "Tests proper UTF-8 encoding support in HTTP responses"
        });
        json_response(&unicode_test)
    }).await;

    // Personalized greeting endpoint
    app.get("/hello", |_req, _params| {
        json_response(&serde_json::json!({
            "message": "Hello from RustFlask! 🚀",
            "framework": "RustFlask",
            "version": "0.1.0",
            "encoding": "UTF-8",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "features": ["async", "methods", "parameters", "json"]
        }))
    }).await;

    // URL parameters demo
    app.get("/hello/{name}", |_req, params| {
        let name = params.params.get("name").map(|s| s.as_str()).unwrap_or("World");
        text_response(&format!("🎉 Hello, {}! Welcome to the RustFlask demo! 🦀

✅ UTF-8 encoding: {}
✅ URL parameters working: {}
✅ Text responses working: {}

Try more endpoints to see all features!", 
            name, "✅", "✅", "✅"))
    }).await;

    // Advanced URL parameters
    app.get("/users/{id}/posts/{post_id}", |_req, params| {
        let user_id = params.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        let post_id = params.params.get("post_id").map(|s| s.as_str()).unwrap_or("unknown");
        
        json_response(&serde_json::json!({
            "success": true,
            "message": "🎯 Multiple URL parameters captured!",
            "parameters": {
                "user_id": user_id,
                "post_id": post_id
            },
            "demonstration": "Complex URL routing with UTF-8 responses",
            "encoding": "UTF-8",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }).await;

    // JSON endpoint
    app.get("/json", |_req, _params| {
        json_response(&serde_json::json!({
            "framework": "RustFlask",
            "version": "0.1.0",
            "description": "A modern async web framework with UTF-8 support",
            "features": {
                "http_methods": true,
                "url_parameters": true,
                "json_serialization": true,
                "configuration": true,
                "async_await": true,
                "utf8_encoding": true,
                "memory_safety": true,
                "type_safety": true
            },
            "demo_shown": "json_response with UTF-8 encoding",
            "encoding": "UTF-8",
            "success": true,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }).await;

    // POST endpoint
    app.post("/echo", |_req, _params| {
        text_response("📬 POST request received successfully! 🎊

✅ This demonstrates:
• POST method handling
• UTF-8 text responses
• Request processing
• Framework versatility

Your POST request was processed correctly!")
    }).await;

    // Server status
    app.get("/status", |_req, _params| {
        json_response(&serde_json::json!({
            "server": "running",
            "framework": "RustFlask",
            "version": "0.1.0",
            "encoding": "UTF-8",
            "features_active": ["UTF-8_encoding", "async_await", "json_serialization", "url_parameters", "http_methods"],
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "memory_safety": "✅",
            "type_safety": "✅"
        }))
    }).await;

    // Configuration info
    let config_clone = config.clone();
    app.get("/config", move |_req, _params| {
        json_response(&serde_json::json!({
            "configuration": {
                "debug_mode": config_clone.debug,
                "host": config_clone.host,
                "port": config_clone.port,
                "framework": "RustFlask",
                "runtime": "Tokio",
                "http_server": "Hyper",
                "json_library": "Serde",
                "charset": "UTF-8"
            },
            "features": {
                "memory_safety": "✅ Rust compiler guarantee",
                "type_safety": "✅ Strong static typing",
                "async_support": "✅ Built on Tokio",
                "utf8_encoding": "✅ Full Unicode support",
                "configuration": "✅ Flexible settings",
                "error_handling": "✅ Proper error management"
            }
        }))
    }).await;

    // Catch-all route
    app.get("/*", |_req, params| {
        let path = params.params.get("*").map(|s| s.as_str()).unwrap_or("unknown");
        let help_text = format!(r#"🔍 Route "{}" not found

💡 Available endpoints:

🌐 Web Pages:
  GET /       - Homepage with demo documentation
  GET /html   - Beautiful HTML page with UTF-8
  GET /utf8   - Unicode character test

🔌 API Endpoints:
  GET /hello               - JSON greeting
  GET /hello/:name         - Personalized greeting  
  GET /json                - JSON demo
  GET /users/:id/posts/:post_id - Multi-parameter demo
  POST /echo               - POST method demo
  GET /status              - Server status
  GET /config              - Configuration info

✨ All responses use UTF-8 encoding for proper Unicode support!"#, 
            path);

        Response::builder()
            .status(404)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from(help_text))
            .unwrap()
    }).await;

    println!("\n🚀 RustFlask Complete Demo v0.1.0\n");
    println!("🌐 Server running on: http://{}/", config.host);
    println!("📍 Port: {}", config.port);
    println!("🐛 Debug mode: {}", config.debug);
    println!("🌍 UTF-8 encoding: ✅ ENABLED");
    println!("");
    println!("📚 Try these URLs:");
    println!("   http://localhost:{}/            - Homepage", config.port);
    println!("   http://localhost:{}/utf8        - Unicode test", config.port);
    println!("   http://localhost:{}/hello/World - Personalized", config.port);
    println!("   http://localhost:{}/json        - JSON demo", config.port);
    println!("   http://localhost:{}/status      - Server status", config.port);
    println!("");
    println!("💻 Command line:");
    println!("   curl http://localhost:{}/hello/世界", config.port);
    println!("   curl -X POST http://localhost:{}/echo", config.port);
    println!("   curl http://localhost:{}/users/rust/posts/tutorial", config.port);
    println!("");

    app.run([127, 0, 0, 1], config.port).await;
}