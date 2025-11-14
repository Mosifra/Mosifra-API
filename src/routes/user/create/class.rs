use rocket::{http::Status, serde::json::Json};

use crate::{structs::class::Class, traits::db::Db};

use super::domain::{CreateClassPayload, CreateClassResponse};

#[post("/university/create_class", data = "<create_class_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_class(
	create_class_payload: Json<CreateClassPayload>,
) -> Result<Json<CreateClassResponse>, Status> {
	let class = Class::try_from(create_class_payload.into_inner())?;

	let is_inserted = class.insert().await;

	Ok(Json(CreateClassResponse {
		success: is_inserted.is_ok(),
	}))
}
