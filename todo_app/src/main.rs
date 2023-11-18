mod db;

use askama::Template;
use axum::{
    response::Html,
    routing::get,
    Router, extract::Path,
};

#[tokio::main]
async fn main() {
    const PORT: i32 = 3000;

    println!("starting on port {PORT}");
    axum::Server::bind(&format!("0.0.0.0:{PORT}").parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/healthcheck", get(|| async { "ok" }))
        .route("/greet/:name", get(greet))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hi</h1>")
}

async fn greet(Path(name): Path<String>) -> HelloTemplate {
    HelloTemplate { name }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn healthcheck_test() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/healthcheck").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn greet_test() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/greet/foobar").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"<h1>Hello, foobar!</h1>");
    }
}