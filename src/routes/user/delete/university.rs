use rocket::{http::Status, serde::json::Json};

use crate::{
	models::{auth::AuthGuard, users::University},
	postgres::Db,
};

use super::domain::{DeleteUniversityPayload, DeleteUniversityResponse};

#[utoipa::path(
	delete,
	path = "/user/university",
	request_body = DeleteUniversityPayload,
	responses(
		(status = 200, description = "University deleted successfully", body = DeleteUniversityResponse),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal Server Error")
	),
	security(
		("api_key" = [])
	),
	tag = "User"
)]
#[delete("/user/university", data = "<delete_university_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn delete_university(
	delete_university_payload: Json<DeleteUniversityPayload>,
	auth: AuthGuard,
) -> Result<Json<DeleteUniversityResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_admin() {
		let university = University::from_id(delete_university_payload.id.clone()).await?;
		Ok(Json(DeleteUniversityResponse {
			success: university.delete().await.is_ok(),
		}))
	} else {
		Err(Status::Unauthorized)
	}
}
