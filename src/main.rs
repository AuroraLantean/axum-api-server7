pub use self::error::{Error, Result};//redeclare Error and Result... to be used as "use crate::{..}"
use crate::db::init_db;
//use anyhow::{Ok, Result};
use axum::{middleware, response::Response, Extension, Router}; use model::ModelController;
//response::Html, routing::get,
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use view::routes_hello;

mod db;
mod rest;
mod view;
mod error;
mod model;
mod web;
mod ctx;

//To be continued: https://crates.io/crates/httpc-test, https://www.youtube.com/watch?v=XZtlD_m59sM, https://www.youtube.com/watch?v=JUWSy9pXgMQ&t=2407s

// cargo watch -q -c -w src/ -x run
#[tokio::main]
async fn main() -> Result<()> {
    // use anyhow to bubble up any error
    // Load environment variables from .env if available
    dotenvy::dotenv().expect("Unable to access .env file");
    //set variables from enviroment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DB_RAM_URL").expect("DB URL not found in env file");

    // Initialize the database and obtain a connection pool. It is reference(Arc) behind the scene, so Arc makes sure this server there is ONLY ONE connection_pool to be shared across this server
    let connection_pool = init_db().await.expect("connection pool is not available");

    //add our tcp listener
    let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not add tcp listener");
    println!("listening on {}", listener.local_addr().unwrap());

    // Initialize ModelController.
	let mc = ModelController::new().await?;

	let routes_apis = web::routes_tickets::routes(mc.clone())
		.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    //use route_layer because we only want this to be applicable to this route
    
    // Initialize the Axum routing service
    let router = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
			mc.clone(),
			web::mw_auth::mw_ctx_resolver,
		))//must be above CookieManager because...
        .layer(CookieManagerLayer::new())
        // Nest service allows you to attach another router to a URL base. So "/" inside the service will be "/books" to the outside world.
        .nest_service("/books", rest::books_service())
        // Add the web view
        .nest_service("/", view::view_service())
        // Add the connection pool as a "layer", available for dependency injection.
        .layer(Extension(connection_pool));
    //layer adds dependency injection layer to it
    
    axum::serve(listener, router.into_make_service())
        .await
        .expect("Error serving application");
    Ok(())
}

async fn main_response_mapper(
//	ctx: Option<Ctx>,
//	uri: Uri,
//	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
	//let uuid = Uuid::new_v4();
    println!();
    res
}