use rocket::{http::Status, serde::json::Json};

use crate::{
	models::{auth::AuthGuard, courses::Class},
	postgres::Db,
};

use super::domain::{CreateClassPayload, CreateClassResponse};

#[post("/create/class", data = "<create_class_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_class(
	auth: AuthGuard,
	create_class_payload: Json<CreateClassPayload>,
) -> Result<Json<CreateClassResponse>, Status> {
	let class = Class::try_from_payload(create_class_payload.into_inner(), auth.session_id)?;

	let is_inserted = class.insert().await;

	Ok(Json(CreateClassResponse {
		success: is_inserted.is_ok(),
	}))
}
