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

    let app = Router::new()
        .route("/", get(handler))
        .route("/healthcheck", get(|| async { "ok" }))
        .route("/greet/:name", get(greet));

    println!("starting on port {PORT}");
    axum::Server::bind(&format!("0.0.0.0:{PORT}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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