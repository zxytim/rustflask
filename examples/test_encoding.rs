use rust_flask::{FlaskApp, FlaskConfig, text_response, json_response};
use hyper::{Response, Body};

#[tokio::main]
async fn main() {
    let config = FlaskConfig {
        debug: true,
        host: "127.0.0.1".to_string(),
        port: 8001,
    };

    let app = FlaskApp::with_config(config.clone());

    // UTF-8 test endpoint
    app.get("/utf8", |_req, _params| {
        Response::builder()
            .status(200)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(Body::from(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>UTF-8 Test</title>
</head>
<body>
    <h1>🎉 UTF-8 Encoding Success! 🎉</h1>
    <p>🌟 This page displays Unicode characters correctly.</p>
    <p>✅ Emojis: 🚀🦀🎯🌍</p>
    <p>✅ Accents: éüñßç</p>
    <p>✅ Chinese: 你好世界</p>
    <p>✅ Arabic: مرحبا</p>
    <p>✅ Russian: привет</p>
</body>
</html>
            "#))
            .unwrap()
    }).await;

    // Text endpoint with UTF-8
    app.get("/text", |_req, _params| {
        text_response("🎉 UTF-8 Text Success! 🎉
✅ This demonstrates proper UTF-8 encoding for text/plain responses.
🌍 International characters: 你好世界, éüñßç, प्रिय, أهلا")
    }).await;

    // JSON endpoint with UTF-8
    app.get("/json", |_req, _params| {
        json_response(&serde_json::json!({
            "success": true,
            "message": "🎉 UTF-8 JSON Success!",
            "unicode_test": {
                "emojis": ["🚀", "🦀", "🎯", "🌍"],
                "accents": ["é", "ü", "ñ", "ß", "ç"],
                "chinese": "你好世界",
                "arabic": "مرحبا",
                "russian": "привет"
            },
            "encoding": "UTF-8",
            "content_type": "application/json; charset=utf-8"
        }))
    }).await;

    println!("🌍 UTF-8 Encoding Test Server");
    println!("🌐 Open http://localhost:8001/utf8");
    println!("📝 Open http://localhost:8001/text");
    println!("📊 Open http://localhost:8001/json");

    app.run([127, 0, 0, 1], config.port).await;
}