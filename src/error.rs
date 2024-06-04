use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
//#[serde(tag = "type", content = "data")]
pub enum Error {
	LoginFail,
	
	// -- Model errors.
	TicketDeleteFailIdNotFound { id: u64 },
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}
impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// implement IntoResponse for Axum!
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<12} - {self:?}", "INTO_RES");

		(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
		// Create a placeholder Axum reponse.
		/*let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);
		
		response */
	}
}