use rust_flask::{FlaskApp, text_response, json_response};
use hyper::{Method, Response, Body, StatusCode};

#[tokio::main]
async fn main() {
    let app = FlaskApp::new();

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

    app.run([127, 0, 0, 1], 8080).await;
}