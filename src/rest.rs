use crate::db::{all_books, book_by_id, Book, Output};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router};
use sqlx::SqlitePool;
/// Build the books REST service.
/// Placing it in its own module with a single service export
/// allows for clean separation of responsibility.
pub fn books_service() -> Router {
    Router::new()
        .route("/", get(get_all_books))
        .route("/:id", get(get_book))
        .route("/add", post(add_book))
        .route("/update", put(update_book))
        .route("/delete/:id", delete(delete_book))
}

/// Wrap the db layer in a GET request, using Axum's built-in JSON support.
///
/// ## Arguments
/// * `Extension(connpool)` - dependency injected by Axum from the database layer.
///
/// ## Returns
/// Either an error 500, or a JSON list of all books in the database.
async fn get_all_books(
    Extension(connpool): Extension<SqlitePool>,
) -> Result<Json<Vec<Book>>, StatusCode> {
    if let Ok(books) = all_books(&connpool).await {
        Ok(Json(books))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

/// Gets a single book.
/// ## Arguments
/// * `Extension(connpool)` - dependency injected by Axum from the database layer.
/// * `Path(id)` - id number, parsed by Axum from the path.
///
/// ## Returns
/// Either a 500 status code, or a JSON encoded book.
async fn get_book(
    Extension(connpool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Book>, StatusCode> {
    if let Ok(book) = book_by_id(&connpool, id).await {
        Ok(Json(book))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

/// Add a book to the database.
/// ## Arguments
/// * `Extension(connpool)` - dependency injected by Axum from the database layer.
/// * A Json-encoded book extracted from the post body.
async fn add_book(
    Extension(connpool): Extension<SqlitePool>,
    extract::Json(book): extract::Json<Book>,
) -> Result<Json<Output>, StatusCode> {
    if let Ok(new_id) = crate::db::add_book(&connpool, &book.title, &book.author).await {
        Ok(Json(Output {
            id: new_id,
            mesg: "".to_owned(),
        }))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

/// Update a book with a patch request
/// ## Arguments
/// * `Extension(connpool)` - dependency injected by Axum from the database layer.
/// * `book` - JSON encoded book to update, from the patch body.
async fn update_book(
    Extension(connpool): Extension<SqlitePool>,
    extract::Json(book): extract::Json<Book>,
) -> StatusCode {
    if crate::db::update_book(&connpool, &book).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

/// Delete a book
/// ## Arguments
/// * `Extension(connpool)` - dependency injected by Axum from the database layer.
/// * `id` of the book to delete, extracted from the URL of the delete call.
async fn delete_book(
    Extension(connpool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> StatusCode {
    if crate::db::delete_book(&connpool, id).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
/*
#[cfg(test)]
mod test {
    use super::*;
    use axum::http::StatusCode;
    use axum::Router;
    use axum_test_helper::TestClient;

    async fn setup_tests() -> TestClient {
        dotenvy::dotenv().ok();
        let connection_pool = crate::init_db().await.unwrap();

        //let app = Router::new().route("/", get(|| async {}));
        let app = crate::router(connection_pool);
        TestClient::new(app.into_make_service())
    } //YouTube: Mastering Rust Web Services from Axum to CRUD Operations@39:00 https://github.com/cloudwalk/axum-test-helper/blob/main/examples/hello-world/src/main.rs

    #[tokio::test]
    async fn get_all_books() {
        let client = setup_tests().await;
        let res = client.get("/books").send().await;
        assert_eq!(res.status().as_str(), StatusCode::OK.as_str());
        let books: Vec<Book> = res.json().await;
        assert!(!books.is_empty());
    }

    #[tokio::test]
    async fn get_one_book() {
        let client = setup_tests().await;
        let res = client.get("/books/1").send().await;
        assert_eq!(res.status().as_str(), StatusCode::OK.as_str());
        let book: Book = res.json().await;
        assert_eq!(book.id, 1)
    }

    #[tokio::test]
    async fn add_book() {
        let client = setup_tests().await;
        let new_book = Book {
            id: -1,
            title: "Test POST Book".to_string(),
            author: "Test POST Author".to_string(),
        };
        let res = client.post("/books/add").json(&new_book).send().await;
        assert_eq!(res.status().as_str(), StatusCode::OK.as_str());
        let new_id: i32 = res.json().await;
        assert!(new_id > 0);

        let test_book = client.get(&format!("/books/{new_id}")).send().await;
        assert_eq!(test_book.status().as_str(), StatusCode::OK.as_str());
        let test_book: Book = test_book.json().await;
        assert_eq!(new_id, test_book.id);
        assert_eq!(new_book.title, test_book.title);
        assert_eq!(new_book.author, test_book.author);
    }

    #[tokio::test]
    async fn update_book() {
        let client = setup_tests().await;
        let mut book1: Book = client.get("/books/1").send().await.json().await;
        book1.title = "Updated book".to_string();
        let res = client.put("/books/edit").json(&book1).send().await;
        assert_eq!(res.status().as_str(), StatusCode::OK.as_str());
        let book2: Book = client.get("/books/1").send().await.json().await;
        assert_eq!(book1.title, book2.title);
    }

    #[tokio::test]
    async fn delete_book() {
        let client = setup_tests().await;
        let new_book = Book {
            id: -1,
            title: "Delete me".to_string(),
            author: "Delete me".to_string(),
        };
        let new_id: i32 = client
            .post("/books/add")
            .json(&new_book)
            .send()
            .await
            .json()
            .await;

        let res = client
            .delete(&format!("/books/delete/{new_id}"))
            .send()
            .await;
        assert_eq!(res.status().as_str(), StatusCode::OK.as_str());

        let all_books: Vec<Book> = client.get("/books").send().await.json().await;
        assert!(all_books.iter().find(|b| b.id == new_id).is_none())
    }
}*/
