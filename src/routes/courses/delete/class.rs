use rocket::{http::Status, serde::json::Json};

use crate::{
	error_handling::StatusOptionHandling,
	models::{
		auth::{AuthGuard, UserType},
		courses::Class,
	},
};

use super::domain::{DeleteClassPayload, DeleteClassResponse};

#[delete("/courses/class", data = "<delete_class_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn delete_class(
	auth: AuthGuard,
	delete_class_payload: Json<DeleteClassPayload>,
) -> Result<Json<DeleteClassResponse>, Status> {
	let payload = delete_class_payload.into_inner();

	if auth.user_type == UserType::University {
		let class = Class::from_id(payload.class_id)
			.await?
			.internal_server_error("No class with this id")?;

		class.delete().await?;

		Ok(Json(DeleteClassResponse { success: true }))
	} else {
		Err(Status::Unauthorized)
	}
}
