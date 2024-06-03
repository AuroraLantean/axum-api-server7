use axum::extract::{Path, Query};
use axum::routing::{get, get_service};
use axum::Router;
use axum_extra::response::Html;
use serde::Deserialize;
use tower_http::services::ServeDir;

pub fn view_service() -> Router {
    Router::new()
    .route("/", get(index_page))
    .fallback_service(serve_file())
}

// region:    --- Routes Hello
pub fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/:name", get(handler_hello2))
}
#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

// /hello?name=John
async fn handler_hello(Query(params): Query<HelloParams>) -> String {
    println!("->> handler_hello {params:?}");
    let name = params.name.as_deref().unwrap_or("default name");
    format!("Hello <strong>{name}</strong>")
}

// /hello2/Mike
async fn handler_hello2(Path(name): Path<String>) -> String {
    println!("->> handler_hello2 {name:?}");
    format!("Hello <strong>{name}</strong>")
}

pub fn serve_file() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

const INDEX_PAGE: &str = include_str!("index.html");

async fn index_page() -> Html<&'static str> {
    Html(INDEX_PAGE)
}
