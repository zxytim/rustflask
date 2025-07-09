use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::Deserialize;
use std::fmt;
use std::str;

#[derive(Debug)]
pub struct JsonError {
    msg: String,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSON Error: {}", self.msg)
    }
}

impl JsonError {
    pub fn new(msg: &str) -> Self {
        JsonError {
            msg: msg.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Json<T>(pub T);

impl<T: for<'de> Deserialize<'de>> Json<T> {
    pub async fn from_request(req: &mut Request<Body>) -> Result<Self, JsonError> {
        let body_bytes = hyper::body::to_bytes(req.body_mut()).await
            .map_err(|_| JsonError::new("Failed to read body"))?;
        
        let body_str = str::from_utf8(&body_bytes)
            .map_err(|_| JsonError::new("Body is not valid UTF-8"))?;

        serde_json::from_str(body_str)
            .map(Json)
            .map_err(|e| JsonError::new(&format!("Failed to parse JSON: {}", e)))
    }
}

pub struct FlaskApp {
    routes: Arc<RwLock<HashMap<String, RouteHandler>>>,
    config: FlaskConfig,
}

type RouteHandler = Arc<dyn Fn(Request<Body>, RouteParams) -> Response<Body> + Send + Sync>;

#[derive(Debug, Clone)]
pub struct FlaskConfig {
    pub debug: bool,
    pub host: String,
    pub port: u16,
}

impl Default for FlaskConfig {
    fn default() -> Self {
        FlaskConfig {
            debug: false,
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RouteParams {
    pub params: HashMap<String, String>,
}

impl RouteParams {
    pub fn new() -> Self {
        RouteParams {
            params: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.params.insert(key, value);
    }
}

impl fmt::Display for RouteParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RouteParams {:?}", self.params)
    }
}

impl Default for RouteParams {
    fn default() -> Self {
        Self::new()
    }
}

impl FlaskApp {
    pub fn new() -> Self {
        FlaskApp {
            routes: Arc::new(RwLock::new(HashMap::new())),
            config: FlaskConfig::default(),
        }
    }

    pub fn with_config(config: FlaskConfig) -> Self {
        FlaskApp {
            routes: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    pub async fn route<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request<Body>, RouteParams) -> Response<Body> + Send + Sync + 'static,
    {
        let mut routes = self.routes.write().await;
        routes.insert(path.to_string(), Arc::new(handler));
    }

    pub async fn get<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request<Body>, RouteParams) -> Response<Body> + Send + Sync + 'static,
    {
        let full_path = format!("GET {}", path);
        self.route(&full_path, move |req, params| {
            if req.method() == Method::GET {
                handler(req, params)
            } else {
                Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Body::from("Method not allowed"))
                    .unwrap()
            }
        }).await;
    }

    pub async fn post<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request<Body>, RouteParams) -> Response<Body> + Send + Sync + 'static,
    {
        let full_path = format!("POST {}", path);
        self.route(&full_path, move |req, params| {
            if req.method() == Method::POST {
                handler(req, params)
            } else {
                Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Body::from("Method not allowed"))
                    .unwrap()
            }
        }).await;
    }

    pub async fn run(&self, addr: [u8; 4], port: u16) {
        let routes = Arc::clone(&self.routes);
        
        let make_svc = make_service_fn(move |_conn| {
            let routes = Arc::clone(&routes);
            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let routes = Arc::clone(&routes);
                    async move {
                        Self::handle_request(req, routes).await
                    }
                }))
            }
        });

        let addr_str = format!("{}.{}.{}.{}:{}", addr[0], addr[1], addr[2], addr[3], port);
        let addr = addr_str.parse().unwrap();

        let server = Server::bind(&addr).serve(make_svc);
        
        println!("Running on http://{}", addr);
        
        if self.config.debug {
            println!("Debug mode enabled");
        }
        
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }
    }

    pub fn config(&self) -> &FlaskConfig {
        &self.config
    }

    async fn handle_request(
        req: Request<Body>,
        routes: Arc<RwLock<HashMap<String, RouteHandler>>>,
    ) -> Result<Response<Body>, hyper::Error> {
        let routes = routes.read().await;
        let method = req.method();
        let path = req.uri().path();
        
        // Try exact method + path match first
        let method_path = format!("{} {}", method, path);
        if let Some(handler) = routes.get(&method_path) {
            let params = Self::extract_path_params(path, path);
            return Ok(handler(req, params));
        }
        
        // Try path-only match for generic routes
        if let Some(handler) = routes.get(path) {
            let params = Self::extract_path_params(path, path);
            return Ok(handler(req, params));
        }
        
        // Handle route parameters (e.g., /users/{id})
        for (pattern, handler) in routes.iter() {
            if let Some(params) = Self::match_route_pattern(pattern, path, method) {
                return Ok(handler(req, params));
            }
        }
        
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap())
    }

    fn match_route_pattern(pattern: &str, path: &str, method: &Method) -> Option<RouteParams> {
        // Handle method-specific patterns
        let actual_pattern = if pattern.starts_with("GET ") 
            || pattern.starts_with("POST ") 
            || pattern.starts_with("PUT ") 
            || pattern.starts_with("DELETE ") {
            &pattern[method.as_str().len() + 1..]
        } else {
            pattern
        };
        
        let mut params = RouteParams::new();
        let pattern_parts: Vec<&str> = actual_pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();
        
        if pattern_parts.len() != path_parts.len() {
            return None;
        }
        
        let mut matches = true;
        for (i, pattern_part) in pattern_parts.iter().enumerate() {
            if let Some(path_part) = path_parts.get(i) {
                if pattern_part.starts_with('{') && pattern_part.ends_with('}') {
                    let param_name = &pattern_part[1..pattern_part.len() - 1];
                    params.insert(param_name.to_string(), path_part.to_string());
                } else if pattern_part != path_part {
                    matches = false;
                    break;
                }
            } else {
                matches = false;
                break;
            }
        }
        
        if matches { Some(params) } else { None }
    }

    fn extract_path_params(route_pattern: &str, actual_path: &str) -> RouteParams {
        let mut params = RouteParams::new();
        
        let pattern_parts: Vec<&str> = route_pattern.split('/').collect();
        let path_parts: Vec<&str> = actual_path.split('/').collect();
        
        if pattern_parts.len() != path_parts.len() {
            return params;
        }
        
        for (i, pattern_part) in pattern_parts.iter().enumerate() {
            if let Some(path_part) = path_parts.get(i) {
                if pattern_part.starts_with('{') && pattern_part.ends_with('}') {
                    let param_name = &pattern_part[1..pattern_part.len() - 1];
                    params.insert(param_name.to_string(), path_part.to_string());
                }
            }
        }
        
        params
    }
}

impl Default for FlaskApp {
    fn default() -> Self {
        Self::new()
    }
}

pub fn text_response(content: &str) -> Response<Body> {
    Response::builder()
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Body::from(content.to_string()))
        .unwrap()
}

pub fn json_response<T: serde::ser::Serialize>(data: &T) -> Response<Body> {
    let json = serde_json::to_string(data).unwrap();
    Response::builder()
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(json))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::Method;

    #[test]
    fn test_text_response() {
        let response = text_response("Hello, World!");
        assert_eq!(response.headers()["Content-Type"], "text/plain; charset=utf-8");
    }

    #[test]
    fn test_json_response() {
        let data = vec!["hello", "world"];
        let response = json_response(&data);
        assert_eq!(response.headers()["Content-Type"], "application/json; charset=utf-8");
    }

    #[tokio::test]
    async fn test_route_handling() {
        let app = FlaskApp::new();
        app.route("/", |req, _params| {
            if req.method() == Method::GET {
                text_response("Hello, World!")
            } else {
                Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Body::empty())
                    .unwrap()
            }
        }).await;

        let test_req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let routes = Arc::clone(&app.routes);
        let response = FlaskApp::handle_request(test_req, routes).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = FlaskApp::new();
        let test_req = Request::builder()
            .method(Method::GET)
            .uri("/nonexistent")
            .body(Body::empty())
            .unwrap();

        let routes = Arc::clone(&app.routes);
        let response = FlaskApp::handle_request(test_req, routes).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_url_parameters() {
        let app = FlaskApp::new();
        app.route("/users/{id}", |req, params| {
            if req.method() == Method::GET {
                let user_id = params.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
                text_response(&format!("User ID: {}", user_id))
            } else {
                Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Body::empty())
                    .unwrap()
            }
        }).await;

        let test_req = Request::builder()
            .method(Method::GET)
            .uri("/users/123")
            .body(Body::empty())
            .unwrap();

        let routes = Arc::clone(&app.routes);
        let response = FlaskApp::handle_request(test_req, routes).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_method_helper() {
        let app = FlaskApp::new();
        app.get("/hello", |_req, _params| {
            text_response("GET Hello")
        }).await;

        let test_req = Request::builder()
            .method(Method::GET)
            .uri("/hello")
            .body(Body::empty())
            .unwrap();

        let routes = Arc::clone(&app.routes);
        let response = FlaskApp::handle_request(test_req, routes).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_post_method_helper() {
        let app = FlaskApp::new();
        app.post("/data", |_req, _params| {
            text_response("POST Data")
        }).await;

        let test_req = Request::builder()
            .method(Method::POST)
            .uri("/data")
            .body(Body::empty())
            .unwrap();

        let routes = Arc::clone(&app.routes);
        let response = FlaskApp::handle_request(test_req, routes).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
