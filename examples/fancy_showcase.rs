use rust_flask::{FlaskApp, FlaskConfig, text_response, json_response};
use hyper::{Method, Response, Body};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = FlaskConfig {
        debug: true,
        host: "127.0.0.1".to_string(),
        port: 8082,
    };

    let app = FlaskApp::with_config(config.clone());

    // HOME PAGE - Fancy HTML with UTF-8 encoding
    app.get("/", |_req, _params| {
        let home_page = r#"HTTP/1.0 200 OK
Content-Type: text/html; charset=utf-8

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RustFlask Showcase</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            line-height: 1.6;
        }
        .container {
            background: rgba(255, 255, 255, 0.1);
            padding: 30px;
            border-radius: 15px;
            backdrop-filter: blur(10px);
            box-shadow: 0 8px 32px rgba(31, 38, 135, 0.37);
            border: 1px solid rgba(255, 255, 255, 0.18);
        }
        .header {
            text-align: center;
            margin-bottom: 30px;
        }
        .header h1 {
            font-size: 2.5em;
            margin: 0;
            background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        .feature-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin: 30px 0;
        }
        .feature-card {
            background: rgba(255, 255, 255, 0.1);
            padding: 20px;
            border-radius: 10px;
            border-left: 4px solid #4ecdc4;
        }
        .endpoint-list {
            background: rgba(0, 0, 0, 0.1);
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
        }
        .endpoint {
            background: rgba(255, 255, 255, 0.1);
            padding: 10px 15px;
            margin: 10px 0;
            border-radius: 5px;
            display: flex;
            align-items: center;
        }
        .method {
            background: #4ecdc4;
            color: white;
            padding: 2px 8px;
            border-radius: 3px;
            font-size: 0.8em;
            font-weight: bold;
            margin-right: 10px;
        }
        .method.post { background: #ff6b6b; }
        .link {
            color: #4ecdc4;
            text-decoration: none;
            font-weight: bold;
        }
        .link:hover {
            text-decoration: underline;
        }
        .example {
            background: rgba(0, 0, 0, 0.2);
            padding: 15px;
            border-radius: 8px;
            margin: 15px 0;
            border-left: 4px solid #ff6b6b;
        }
        .code {
            font-family: 'Courier New', monospace;
            background: rgba(0, 0, 0, 0.3);
            padding: 15px;
            border-radius: 5px;
            margin: 10px 0;
            overflow-x: auto;
        }
        .curl-example {
            background: rgba(0, 0, 0, 0.4);
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
            border-left: 4px solid #ffd93d;
        }
        .checklist {
            list-style: none;
            padding: 0;
        }
        .checklist li:before {
            content: "✅ ";
            color: #4ecdc4;
            font-weight: bold;
        }
        .footer {
            text-align: center;
            margin-top: 30px;
            padding-top: 20px;
            border-top: 1px solid rgba(255, 255, 255, 0.2);
            font-size: 0.9em;
            opacity: 0.8;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 RustFlask</h1>
            <p><strong>A modern async web framework for Rust</strong></p>
        </div>

        <div class="feature-grid">
            <div class="feature-card">
                <h3>🚀 Async Runtime</h3>
                <p>Built on Tokio and Hyper for maximum performance with async/await support.</p>
            </div>
            <div class="feature-card">
                <h3>🔧 HTTP Methods</h3>
                <p>Clean API with dedicated helpers for GET, POST and other HTTP methods.</p>
            </div>
            <div class="feature-card">
                <h3>🌐 URL Parameters</h3>
                <p>Dynamic routing with `{param}` syntax for clean RESTful APIs.</p>
            </div>
            <div class="feature-card">
                <h3>📊 JSON Support</h3>
                <p>Built-in JSON serialization with Serde for APIs and responses.</p>
            </div>
        </div>

        <div class="endpoint-list">
            <h2>📡 Available Endpoints</h2>
            
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/hello" class="link">/hello</a> - Simple greeting
            </div>
            
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/hello/World" class="link">/hello/{name}</a> - Personalized greeting
            </div>
            
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/json" class="link">/json</a> - JSON response example
            </div>
            
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/users/123/posts/456" class="link">/users/{id}/posts/{post_id}</a> - Multiple parameters
            </div>
            
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/info" class="link">/info</a> - Framework information
            </div>
            
            <div class="endpoint">
                <span class="method post">POST</span>
                <a href="/echo" class="link">/echo</a> - POST method demo
            </div>
            
            <div class="endpoint">
                <span class="method">GET</span>
                <a href="/debug" class="link">/debug</a> - Debug mode info
            </div>
        </div>

        <div class="example">
            <h3>🎯 Quick Examples</h3>
            <div class="curl-example">
                <h4>Personalize your greeting:</h4>
                <div class="code">
curl http://localhost:8082/hello/YourName
                </div>
            </div>
            <div class="curl-example">
                <h4>Test URL parameters:</h4>
                <div class="code">
curl http://localhost:8082/users/123/posts/456
                </div>
            </div>
            <div class="curl-example">
                <h4>Try POST method:</h4>
                <div class="code">
curl -X POST http://localhost:8082/echo
                </div>
            </div>
        </div>

        <div class="feature-card">
            <h3>🛠️ Framework Features</h3>
            <ul class="checklist">
                <li>Async/Await Support</li>
                <li>HTTP Method Routing</li>
                <li>URL Parameter Extraction</li>
                <li>JSON Serialization</li>
                <li>Configuration Management</li>
                <li>Error Handling</li>
                <li>Type Safety</li>
            </ul>
        </div>

        <div class="footer">
            <p>🦀 Powered by Rust | Built with ❤️ for the community</p>
            <p>📚 Check out the GitHub repository for more examples and documentation</p>
        </div>
    </div>
</body>
</html>
        "#;
        
        Response::builder()
            .status(200)
            .header("Content-Type", "text/html; charset=utf-8")  
            .body(Body::from(home_page))
            .unwrap()
    }).await;

    // API documentation endpoint
    app.get("/api", |_req, _params| {
        let api_docs = r#"HTTP/1.0 200 OK
Content-Type: application/json

{
  "framework": "RustFlask",
  "version": "0.1.0",
  "endpoints": [
    {
      "method": "GET",
      "path": "/",
      "description": "Fancy HTML home page"
    },
    {
      "method": "GET", 
      "path": "/hello",
      "description": "Simple greeting"
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
      "method": "POST",
      "path": "/echo",
      "description": "POST method demonstration"
    }
  ],
  "features": [
    "HTTP Method Helpers",
    "URL Parameters",
    "JSON Serialization", 
    "Configuration Management",
    "Async/Await",
    "UTF-8 Encoding"
  ]
}
        "#;
        
        Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Body::from(api_docs))
            .unwrap()
    }).await;

    // Simple greeting
    app.get("/hello", |_req, _params| {
        text_response("👋 Hello from RustFlask! 🦀")
    }).await;

    // Personalized greeting with URL parameter
    app.get("/hello/{name}", |_req, params| {
        let name = params.params.get("name").map(|s| s.as_str()).unwrap_or("World");
        text_response(&format!("🎉 Hello, {}! Welcome to RustFlask! 🌟", name))
    }).await;

    // JSON response with data
    app.get("/json", |_req, _params| {
        let data = serde_json::json!({
            "framework": "RustFlask",
            "version": "0.1.0",
            "features": ["async", "methods", "parameters", "json", "utf8"],
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "success": true,
            "encoding": "UTF-8",
            "rust_features": ["memory_safety", "zero_cost_abstractions", "fearless_concurrency"]
        });
        json_response(&data)
    }).await;

    // Complex URL parameters with multiple parameters
    app.get("/users/{id}/posts/{post_id}", |_req, params| {
        let user_id = params.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        let post_id = params.params.get("post_id").map(|s| s.as_str()).unwrap_or("unknown");
        
        let response = serde_json::json!({
            "message": "URL parameters parsed successfully! 🎯",
            "user_id": user_id,
            "post_id": post_id,
            "path_info": format!("User {} -> Post {}", user_id, post_id),
            "parameters": {
                "id": user_id,
                "post_id": post_id
            }
        });
        json_response(&response)
    }).await;

    // POST endpoint
    app.post("/echo", |_req, _params| {
        text_response("📬 POST request received successfully! 🎊\n\n✅ This demonstrates POST method handling in RustFlask.")
    }).await;

    // Interactive playground
    app.get("/playground", |_req, _params| {
        let playground = r#"HTTP/1.0 200 OK
Content-Type: text/html; charset=utf-8

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RustFlask Interactive Playground</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; background: #f0f8ff; }
        .container { background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .endpoint { margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 8px; }
        .method { display: inline-block; background: #007bff; color: white; padding: 5px 10px; border-radius: 5px; font-size: 12px; margin-right: 10px; }
        .method.post { background: #28a745; }
        .code { background: #f8f9fa; padding: 10px; border-radius: 5px; font-family: monospace; margin: 10px 0; }
        .button { background: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; margin: 5px; }
        .button:hover { background: #0056b3; }
        .result { background: #f8f9fa; padding: 15px; border-radius: 5px; margin: 10px 0; border-left: 4px solid #007bff; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎮 RustFlask Interactive Playground</h1>
        <p>Click the buttons below to test different endpoints!</p>
        
        <div class="endpoint">
            <h3>GET /</h3>
            <p>Fancy HTML homepage</p>
            <button class="button" onclick="testGet('/')">Test /</button>
            <div id="result-1" class="result" style="display: none;"></div>
        </div>
        
        <div class="endpoint">
            <h3>GET /hello/YourName</h3>
            <p>URL parameter example</p>
            <input type="text" id="nameInput" placeholder="Your name" value="World">
            <button class="button" onclick="testGet('/hello/' + document.getElementById('nameInput').value)">Test Hello</button>
            <div id="result-2" class="result" style="display: none;"></div>
        </div>
        
        <div class="endpoint">
            <h3>GET /json</h3>
            <p>JSON response</p>
            <button class="button" onclick="testJson('/json')">Test JSON</button>
            <div id="result-3" class="result" style="display: none;"></div>
        </div>
        
        <div class="endpoint">
            <h3>POST /echo</h3>
            <p>POST method demo</p>
            <button class="button" onclick="testPost('/echo')">Test POST</button>
            <div id="result-4" class="result" style="display: none;"></div>
        </div>
        
        <div class="example">
            <h3>📝 Manual cURL Examples</h3>
            <div class="code">
curl http://localhost:8082/hello/RustDeveloper
            </div>
            <div class="code">
curl http://localhost:8082/users/123/posts/456
            </div>
            <div class="code">
curl -X POST http://localhost:8082/echo
            </div>
        </div>
    </div>

    <script>
        function testGet(url) {
            fetch('http://localhost:8082' + url)
                .then(response => response.text())
                .then(data => showResult(url === '/' ? 'result-1' : url.includes('hello') ? 'result-2' : 'result-3', data, 'text'))
                .catch(error => showResult('result-1', 'Error: ' + error, 'text'));
        }
        
        function testJson(url) {
            fetch('http://localhost:8082' + url)
                .then(response => response.json())
                .then(data => showResult('result-3', JSON.stringify(data, null, 2), 'json'))
                .catch(error => showResult('result-3', 'Error: ' + error, 'text'));
        }
        
        function testPost(url) {
            fetch('http://localhost:8082' + url, { method: 'POST' })
                .then(response => response.text())
                .then(data => showResult('result-4', data, 'text'))
                .catch(error => showResult('result-4', 'Error: ' + error, 'text'));
        }
        
        function showResult(elementId, data, type) {
            const element = document.getElementById(elementId);
            element.style.display = 'block';
            if (type === 'json') {
                element.innerHTML = '<pre>' + data + '</pre>';
            } else {
                element.textContent = data;
            }
        }
    </script>
</body>
</html>
        "#;
        
        Response::builder()
            .status(200)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(Body::from(playground))
            .unwrap()
    }).await;

    // Test parameters
    app.get("/params/{param1}/{param2}/{param3}", |_req, params| {
        let mut params_map = HashMap::new();
        for (key, value) in &params.params {
            params_map.insert(key.clone(), value.clone());
        }
        
        let response = serde_json::json!({
            "message": "🎯 All URL parameters captured!",
            "parameters": params_map,
            "count": params_map.len(),
            "all_params": params_map.keys().collect::<Vec<_>>(),
            "note": "This endpoint demonstrates capturing multiple URL parameters"
        });
        json_response(&response)
    }).await;

    // Alive endpoint for health checks
    app.get("/alive", |_req, _params| {
        json_response(&serde_json::json!({
            "status": "alive",
            "timestamp": "2024-01-01T12:00:00Z",
            "framework": "RustFlask",
            "message": "🟢 All systems operational!",
            "uptime": "running"
        }))
    }).await;

    // Print startup banner
    println!("\n🚀🦀 RUSTFLASK SHOWCASE 🦀🚀");
    println!("================================================================");
    println!("🌐 Server running on: http://{}/    📱 Open your browser!", config.host);
    println!("🔧 Port: {}    🐛 Debug mode: {}", config.port, config.debug);
    println!("================================================================");
    println!("💡 QUICK START:");
    println!("   • http://localhost:{}/            - Fancy homepage", config.port);
    println!("   • http://localhost:{}/hello/World - Personalized greeting", config.port);
    println!("   • http://localhost:{}/playground  - Interactive playground", config.port);
    println!("   • http://localhost:{}/alive       - Health check", config.port);
    println!("🎯 ALL FEATURES DEMO:");
    println!("   • HTTP Methods (GET, POST)");
    println!("   • URL Parameters (`{{id}}`, `{{post_id}}`)");
    println!("   • UTF-8 HTML responses");
    println!("   • JSON responses");
    println!("   • Interactive JavaScript playground");
    println!("================================================================\n");

    // Start the server
    app.run([127, 0, 0, 1], config.port).await;
}