use crate::ctx::Ctx;
use axum::{extract::{Path, State}, routing::{delete, post}, Json, Router};
use crate::Result;
use crate::model::{ModelController, Ticket, TicketForCreate};

/* Make sub state
#[derive(Debug, FromRef)]//add axum feature: "macros in cargo.toml"
struct AppState {
	mc: ModelController
	s3connection: S3Connection
}*/

pub fn routes(mc: ModelController) -> Router {
	Router::new()
		.route("/tickets", post(create_ticket).get(list_tickets))
		.route("/tickets/:id", delete(delete_ticket).patch(update_ticket))
		.with_state(mc)
}//with_state(mc) at the end to make mc available to all routes above it

// region:    --- REST Handlers
async fn create_ticket(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
	println!("->> {:<12} - create_ticket", "HANDLER");

	let ticket = mc.create_ticket(ctx, ticket_fc).await?;

	Ok(Json(ticket))
}

async fn list_tickets(
	State(mc): State<ModelController>,
	ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
	println!("->> {:<12} - list_tickets", "HANDLER");

	let tickets = mc.list_tickets(ctx).await?;

	Ok(Json(tickets))
}

async fn delete_ticket(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
	println!(">>> {:<12} - delete_ticket", "HANDLER");

	let ticket = mc.delete_ticket(ctx, id).await?;

	Ok(Json(ticket))
}

async fn update_ticket(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
	println!(">>> {:<12} - update_ticket", "HANDLER");

	let ticket = mc.update_ticket(ctx, id).await?;

	Ok(Json(ticket))
}
// endregion: --- REST Handlers
