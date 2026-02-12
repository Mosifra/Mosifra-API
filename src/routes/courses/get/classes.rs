use rocket::{http::Status, serde::json::Json};

use crate::models::{auth::AuthGuard, courses::dto::class::ClassDto};

use super::domain::GetClassesResponse;

#[utoipa::path(
	get,
	path = "/courses/classes",
	responses(
		(status = 200, description = "List of classes", body = GetClassesResponse),
		(status = 401, description = "Unauthorized")
	),
	security(
		("api_key" = [])
	),
	tag = "Courses"
)]
#[get("/courses/classes")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_classes(auth: AuthGuard) -> Result<Json<GetClassesResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_university() {
		let university = generic_user.to_university()?;
		let classes = university.get_classes().await?;

		Ok(Json(GetClassesResponse {
			success: true,
			classes: Some(ClassDto::from_vec(classes)),
		}))
	} else {
		Err(Status::Unauthorized)
	}
}
