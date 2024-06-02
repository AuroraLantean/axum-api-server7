use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use axum_extra::response::Html;
use serde::Deserialize;

pub fn view_service() -> Router {
    Router::new()
    .route("/", get(index_page))
    .route("/hello",
        get(handler_hello),
    )
}
#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> String {
    println!("->> handler_hello {params:?}");
    let name = params.name.as_deref().unwrap_or("default name");
    format!("Hello <strong>{name}</strong>")
}

const INDEX_PAGE: &str = include_str!("index.html");

async fn index_page() -> Html<&'static str> {
    Html(INDEX_PAGE)
}
