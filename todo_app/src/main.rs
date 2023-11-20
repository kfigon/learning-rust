mod db;

use std::{sync::{Mutex, Arc}, collections::HashMap};

use askama::Template;
use axum::{
    response::Html,
    routing::{get,post},
    Router, extract::{Path, State}, Form,
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

type Db = Arc<Mutex<Vec<String>>>;

fn app() -> Router {
    let db: Db = Arc::new(Mutex::new(vec![]));

    Router::new()
        .route("/", get(handler))
        .route("/healthcheck", get(|| async { "ok" }))
        .route("/greet/:name", get(greet))
        .route("/all", get(list_all))

        .route("/main", get(main_page))
        .route("/clicked", post(clicked))
        .with_state(db)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hi</h1>")
}

async fn greet(
    Path(name): Path<String>,
    State(db): State<Db>
) -> HelloTemplate {
    let mut v = db.lock().unwrap();
    v.push(name.clone());

    HelloTemplate { name }
}

async fn list_all(
    State(db): State<Db>
) -> SummaryTemplate {
    let v = db.lock().unwrap();
    SummaryTemplate { names: v.clone() }
}

async fn main_page() -> MainPage {
    MainPage { button_div: ButtonTemplate { count: 0 } }
}

async fn clicked(
    Form(f): Form<HashMap<String, String>>
) -> ButtonTemplate {
    let count = f.get("v")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or_default() + 1;
    ButtonTemplate { count }
}

#[derive(Template)]
#[template(path = "button.html")]
struct ButtonTemplate {
    count: usize
}

#[derive(Template)]
#[template(path = "main.html")]
struct MainPage {
    button_div: ButtonTemplate
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String
}

#[derive(Template)]
#[template(path = "all.html")]
struct SummaryTemplate {
    names: Vec<String>
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
        
        let body = String::from_utf8(
            hyper::body::to_bytes(response.into_body())
            .await
            .unwrap()
            .to_vec()
        ).unwrap();
        
        assert!(body.contains("<h1>Hello, foobar!</h1>"));
    }


    #[tokio::test]
    async fn invalid_url_test() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/foobarz").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}