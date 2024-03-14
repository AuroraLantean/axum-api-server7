use axum::routing::get;
use axum::Router;
use axum_extra::response::Html;
//use axum::response::Html;

pub fn view_service() -> Router {
    Router::new().route("/", get(index_page)).route(
        "/hello",
        get(|| async { Html("Hello <strong>world</strong") }),
    )
}

const INDEX_PAGE: &str = include_str!("index.html");

async fn index_page() -> Html<&'static str> {
    Html(INDEX_PAGE)
}
