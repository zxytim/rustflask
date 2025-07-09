use rust_flask::{FlaskApp, FlaskConfig, json_response};
use hyper::{Response, Body};

#[tokio::main]
async fn main() {
    let config = FlaskConfig {
        debug: true,
        host: "127.0.0.1".to_string(),
        port: 8084,
    };

    let app = FlaskApp::with_config(config.clone());

    // Homepage with proper UTF-8 encoding
    app.get("/", |_req, _params| {
        let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Executive Briefing | RustFlask Framework</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --primary: #1a1a2e;
            --secondary: #16213e;
            --accent: #e94560;
            --accent-secondary: #f39c12;
            --success: #27ae60;
            --light: #f8f9fa;
            --muted: #6c757d;
            --border: #e1e8ed;
            --shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            --shadow-lg: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        }

        * { margin: 0; padding: 0; box-sizing: border-box; }

        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            color: var(--primary);
            background: var(--light);
        }

        .executive-header {
            background: linear-gradient(135deg, var(--primary) 0%, var(--secondary) 100%);
            color: white;
            padding: 60px 0;
            text-align: center;
            position: relative;
            overflow: hidden;
        }

        .executive-header::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 20"><defs><pattern id="grid" width="10" height="10" patternUnits="userSpaceOnUse"><path d="M 10 0 L 0 0 0 10" fill="none" stroke="rgba(255,255,255,0.05)" stroke-width="1"/></pattern></defs><rect width="100" height="20" fill="url(%23grid)"/></svg>');
            opacity: 0.3;
        }

        .executive-content {
            position: relative;
            z-index: 1;
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 20px;
        }

        .executive-title {
            font-size: 3.5rem;
            font-weight: 700;
            margin-bottom: 1rem;
            background: linear-gradient(45deg, #fff, #e94560);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .executive-subtitle {
            font-size: 1.3rem;
            font-weight: 300;
            opacity: 0.9;
            margin-bottom: 2rem;
            max-width: 600px;
            margin-left: auto;
            margin-right: auto;
        }

        .kpi-container {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 30px;
            margin: 40px 0;
        }

        .kpi-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 30px;
            text-align: center;
            border: 1px solid rgba(255, 255, 255, 0.2);
        }

        .kpi-metric {
            font-size: 2.5rem;
            font-weight: 700;
            color: var(--accent);
            margin-bottom: 8px;
        }

        .kpi-label {
            font-size: 0.9rem;
            opacity: 0.8;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .main-container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 80px 20px;
        }

        .executive-summary {
            background: white;
            border-radius: 20px;
            padding: 60px;
            margin-bottom: 60px;
            box-shadow: var(--shadow-lg);
            border: 1px solid var(--border);
        }

        .summary-title {
            font-size: 2.5rem;
            font-weight: 600;
            margin-bottom: 30px;
            color: var(--primary);
        }

        .executive-grid {
            display: grid;
            grid-template-columns: 2fr 1fr;
            gap: 60px;
            align-items: start;
        }

        .capabilities-section {
            background: white;
            border-radius: 16px;
            padding: 40px;
            box-shadow: var(--shadow);
            border: 1px solid var(--border);
        }

        .section-header {
            font-size: 2rem;
            font-weight: 600;
            margin-bottom: 30px;
            color: var(--primary);
            display: flex;
            align-items: center;
            gap: 15px;
        }

        .capability-list {
            list-style: none;
            space-y: 20px;
        }

        .capability-item {
            display: flex;
            align-items: flex-start;
            gap: 15px;
            padding: 15px 0;
            border-bottom: 1px solid var(--border);
        }

        .capability-item:last-child {
            border-bottom: none;
        }

        .capability-icon {
            width: 32px;
            height: 32px;
            background: var(--accent);
            color: white;
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.2rem;
            flex-shrink: 0;
        }

        .capability-content h4 {
            font-size: 1.1rem;
            font-weight: 600;
            margin-bottom: 5px;
            color: var(--primary);
        }

        .capability-content p {
            color: var(--muted);
            font-size: 0.95rem;
        }

        .endpoints-sidebar {
            background: white;
            border-radius: 16px;
            padding: 30px;
            box-shadow: var(--shadow);
            border: 1px solid var(--border);
            position: sticky;
            top: 20px;
        }

        .endpoint-list {
            space-y: 15px;
        }

        .endpoint-item {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 12px;
            background: #f8f9fa;
            border-radius: 8px;
            margin-bottom: 10px;
            transition: all 0.3s ease;
        }

        .endpoint-item:hover {
            background: #e9ecef;
            transform: translateX(4px);
        }

        .endpoint-method {
            background: var(--accent);
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: 600;
            min-width: 40px;
            text-align: center;
        }

        .endpoint-method.POST {
            background: var(--accent-secondary);
        }

        .endpoint-url {
            color: var(--primary);
            text-decoration: none;
            font-weight: 500;
            font-family: 'Courier New', monospace;
            font-size: 0.9rem;
        }

        .endpoint-url:hover {
            color: var(--accent);
        }

        .demo-section {
            background: white;
            border-radius: 16px;
            padding: 40px;
            margin: 40px 0;
            box-shadow: var(--shadow);
            border: 1px solid var(--border);
        }

        .demo-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 30px;
            margin-top: 30px;
        }

        .demo-card {
            background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
            border-radius: 12px;
            padding: 25px;
            border-left: 4px solid var(--accent);
        }

        .demo-card h4 {
            font-weight: 600;
            margin-bottom: 10px;
            color: var(--primary);
        }

        .code-example {
            background: var(--primary);
            color: #34d399;
            padding: 15px;
            border-radius: 8px;
            font-family: 'Courier New', monospace;
            font-size: 0.85rem;
            margin: 10px 0;
            overflow-x: auto;
        }

        .architecture-section {
            background: white;
            border-radius: 16px;
            padding: 40px;
            margin: 40px 0;
            box-shadow: var(--shadow);
            border: 1px solid var(--border);
        }

        .tech-stack {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
            gap: 20px;
            margin-top: 30px;
        }

        .tech-item {
            text-align: center;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 12px;
            border: 2px solid transparent;
            transition: all 0.3s ease;
        }

        .tech-item:hover {
            border-color: var(--accent);
            transform: translateY(-2px);
        }

        .tech-icon {
            font-size: 2rem;
            margin-bottom: 10px;
            display: block;
        }

        .tech-name {
            font-weight: 600;
            color: var(--primary);
        }

        .executive-footer {
            background: var(--primary);
            color: white;
            text-align: center;
            padding: 60px 20px;
            margin-top: 80px;
        }

        .footer-content {
            max-width: 1200px;
            margin: 0 auto;
        }

        .footer-title {
            font-size: 2rem;
            font-weight: 600;
            margin-bottom: 20px;
        }

        .footer-subtitle {
            font-size: 1.1rem;
            opacity: 0.9;
            max-width: 600px;
            margin: 0 auto;
        }

        @media (max-width: 768px) {
            .executive-grid {
                grid-template-columns: 1fr;
                gap: 40px;
            }
            
            .executive-title {
                font-size: 2.5rem;
            }
            
            .executive-summary {
                padding: 40px 30px;
            }
            
            .summary-title {
                font-size: 2rem;
            }
            
            .kpi-container {
                grid-template-columns: repeat(2, 1fr);
            }
        }
    </style>
</head>
<body>
    <header class="executive-header">
        <div class="executive-content">
            <h1 class="executive-title">Executive Briefing</h1>
            <p class="executive-subtitle">RustFlask: A Strategic Investment in Next-Generation Web Infrastructure</p>
            
            <div class="kpi-container">
                <div class="kpi-card">
                    <div class="kpi-metric">⚡</div>
                    <div class="kpi-label">Blazing Performance</div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-metric">🛡️</div>
                    <div class="kpi-label">Memory Safety</div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-metric">🌐</div>
                    <div class="kpi-label">UTF-8 Native</div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-metric">🔧</div>
                    <div class="kpi-label">Type Safe</div>
                </div>
            </div>
        </div>
    </header>

    <main class="main-container">
        <section class="executive-summary">
            <h2 class="summary-title">Executive Summary</h2>
            <div class="executive-grid">
                <div>
                    <p style="font-size: 1.1rem; margin-bottom: 20px; color: var(--muted);">
                        RustFlask represents a paradigm shift in web framework architecture, combining the performance benefits of systems programming with the productivity of modern web development.
                    </p>
                    <p style="margin-bottom: 20px;">
                        Built on Rust's zero-cost abstractions and async/await paradigm, this framework delivers unprecedented performance while maintaining memory safety without garbage collection overhead.
                    </p>
                    <p style="margin-bottom: 20px;">
                        The strategic advantage lies in its ability to handle high-throughput scenarios with predictable latency, making it ideal for mission-critical applications where performance and reliability are non-negotiable.
                    </p>
                    <p style="font-weight: 500; color: var(--accent);">
                        Key Business Impact: Reduced infrastructure costs, improved user experience, and enhanced system reliability through memory-safe architecture.
                    </p>
                </div>
                
                <div class="capabilities-section">
                    <h3 class="section-header">
                        <span>🎯</span>
                        Core Capabilities
                    </h3>
                    <ul class="capability-list">
                        <li class="capability-item">
                            <div class="capability-icon">⚡</div>
                            <div class="capability-content">
                                <h4>Async/Await Performance</h4>
                                <p>Built on Tokio runtime for maximum throughput</p>
                            </div>
                        </li>
                        <li class="capability-item">
                            <div class="capability-icon">🛡️</div>
                            <div class="capability-content">
                                <h4>Memory Safety Guarantee</h4>
                                <p>Zero runtime exceptions through Rust's ownership system</p>
                            </div>
                        </li>
                        <li class="capability-item">
                            <div class="capability-icon">🌐</div>
                            <div class="capability-content">
                                <h4>UTF-8 Native Support</h4>
                                <p>Proper Unicode handling for global applications</p>
                            </div>
                        </li>
                        <li class="capability-item">
                            <div class="capability-icon">📊</div>
                            <div class="capability-content">
                                <h4>Type-Safe JSON APIs</h4>
                                <p>Built-in serialization with compile-time safety</p>
                            </div>
                        </li>
                        <li class="capability-item">
                            <div class="capability-icon">🔧</div>
                            <div class="capability-content">
                                <h4>HTTP Method Routing</h4>
                                <p>Clean API design with dedicated method handlers</p>
                            </div>
                        </li>
                        <li class="capability-item">
                            <div class="capability-icon">🔗</div>
                            <div class="capability-content">
                                <h4>URL Parameter Extraction</h4>
                                <p>Dynamic routing with type-safe parameter binding</p>
                            </div>
                        </li>
                    </ul>
                </div>
            </div>
        </section>

        <section class="demo-section">
            <h3 class="section-header">
                <span>🧪</span>
                Live API Demonstrations
            </h3>
            <p style="color: var(--muted); margin-bottom: 30px;">Interactive endpoints showcasing framework capabilities in real-time scenarios.</p>
            
            <div class="demo-grid">
                <div class="demo-card">
                    <h4>🔧 Health & Status Monitoring</h4>
                    <p>System health checks and configuration introspection</p>
                    <div class="code-example">GET /alive</div>
                    <div class="code-example">GET /api</div>
                </div>
                
                <div class="demo-card">
                    <h4>🌐 Personalization Engine</h4>
                    <p>Dynamic content generation with URL parameters</p>
                    <div class="code-example">GET /hello/{name}</div>
                    <div class="code-example">GET /users/{id}/posts/{post_id}</div>
                </div>
                
                <div class="demo-card">
                    <h4>📊 Data Serialization</h4>
                    <p>Type-safe JSON responses with UTF-8 encoding</p>
                    <div class="code-example">GET /json</div>
                    <div class="code-example">POST /echo</div>
                </div>
            </div>
        </section>

        <section class="architecture-section">
            <h3 class="section-header">
                <span>🏗️</span>
                Technical Architecture
            </h3>
            <p style="color: var(--muted); margin-bottom: 30px;">Built on industry-leading technologies for enterprise-grade performance and reliability.</p>
            
            <div class="tech-stack">
                <div class="tech-item">
                    <span class="tech-icon">🦀</span>
                    <div class="tech-name">Rust</div>
                </div>
                <div class="tech-item">
                    <span class="tech-icon">⚡</span>
                    <div class="tech-name">Tokio</div>
                </div>
                <div class="tech-item">
                    <span class="tech-icon">🚀</span>
                    <div class="tech-name">Hyper</div>
                </div>
                <div class="tech-item">
                    <span class="tech-icon">📊</span>
                    <div class="tech-name">Serde</div>
                </div>
                <div class="tech-item">
                    <span class="tech-icon">🌐</span>
                    <div class="tech-name">UTF-8</div>
                </div>
                <div class="tech-item">
                    <span class="tech-icon">🔒</span>
                    <div class="tech-name">Memory Safe</div>
                </div>
            </div>
        </section>

        <div class="endpoints-sidebar">
            <h3 class="section-header">
                <span>🔗</span>
                API Endpoints
            </h3>
            <div class="endpoint-list">
                <div class="endpoint-item">
                    <span class="endpoint-method">GET</span>
                    <a href="/hello" class="endpoint-url">/hello</a>
                </div>
                <div class="endpoint-item">
                    <span class="endpoint-method">GET</span>
                    <a href="/hello/RustDeveloper" class="endpoint-url">/hello/{name}</a>
                </div>
                <div class="endpoint-item">
                    <span class="endpoint-method">GET</span>
                    <a href="/json" class="endpoint-url">/json</a>
                </div>
                <div class="endpoint-item">
                    <span class="endpoint-method">GET</span>
                    <a href="/users/123/posts/456" class="endpoint-url">/users/{id}/posts/{post_id}</a>
                </div>
                <div class="endpoint-item">
                    <span class="endpoint-method">GET</span>
                    <a href="/alive" class="endpoint-url">/alive</a>
                </div>
                <div class="endpoint-item">
                    <span class="endpoint-method POST">POST</span>
                    <a href="/echo" class="endpoint-url">/echo</a>
                </div>
                <div class="endpoint-item">
                    <span class="endpoint-method">GET</span>
                    <a href="/api" class="endpoint-url">/api</a>
                </div>
            </div>
            
            <div style="margin-top: 30px; padding-top: 20px; border-top: 1px solid var(--border);">
                <h4 style="margin-bottom: 15px; color: var(--primary);">Quick Start</h4>
                <div class="code-example" style="font-size: 0.8rem;">curl http://localhost:8084/hello/Executive</div>
                <div class="code-example" style="font-size: 0.8rem;">curl -X POST http://localhost:8084/echo</div>
            </div>
        </div>
    </main>

    <footer class="executive-footer">
        <div class="footer-content">
            <h3 class="footer-title">🦀 RustFlask Enterprise v0.1.0</h3>
            <p class="footer-subtitle">
                Strategic technology investment for organizations requiring maximum performance, 
                uncompromising security, and future-proof architecture.
            </p>
        </div>
    </footer>
</body>
</html>
        "#;

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
        json_response(&serde_json::json!({
            "message": "👋 Hello from RustFlask! 🦀",
            "encoding": "UTF-8",
            "demo": "text_response with emoji support"
        }))
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
        Response::builder()
            .status(200)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from("📬 POST request received successfully! 🎊\n\n✅ This demonstrates POST method handling in RustFlask:\n   • HTTP method routing ✅\n   • Request handling ✅\n   • UTF-8 text responses ✅\n   • Framework versatility ✅"))
            .unwrap()
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
  • GET /users/{{id }}/posts/{{post_id }} - URL parameters
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
    println!("   • URL Parameters ({{}}, {{}}, {{}})"); // {id}, {post_id}, {name}
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