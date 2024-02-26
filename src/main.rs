//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```
mod db;
mod rest;
mod view;
use crate::db::init_db;
//use anyhow::Result;
use axum::{Extension, Router}; //response::Html, routing::get,
use sqlx::SqlitePool;

/// Build the overall web service router.
/// Constructing the router in a function makes it easy to re-use in unit tests.
fn router(connection_pool: SqlitePool) -> Router {
    Router::new()
        // Nest service allows you to attach another router to a URL base.
        // "/" inside the service will be "/books" to the outside world.
        .nest_service("/books", rest::books_service())
        // Add the web view
        .nest_service("/", view::view_service())
        // Add the connection pool as a "layer", available for dependency injection.
        .layer(Extension(connection_pool))
} //layer adds dependency injection layer to it

#[tokio::main]
async fn main() {
    // Load environment variables from .env if available
    dotenvy::dotenv().ok();

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await.expect("connection pool is not available");

    // Initialize the Axum routing service
    let app = router(connection_pool);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
/*let app = create_routes(mode, db_conn);
axum::serve(listener, app.await.into_make_service()).await.unwrap();

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}*/
