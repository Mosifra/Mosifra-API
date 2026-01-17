use rocket::{http::Status, serde::json::Json};

use crate::models::{auth::AuthGuard, users::University};

use super::domain::GetUniversitiesResponse;

#[get("/user/universities")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_universities(auth: AuthGuard) -> Result<Json<GetUniversitiesResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;
	if generic_user.is_admin() {
		let universities = University::get_all().await;

		universities.map_or(
			Ok(Json(GetUniversitiesResponse {
				success: false,
				universities: None,
			})),
			|universities| {
				Ok(Json(GetUniversitiesResponse {
					success: true,
					universities: Some(universities),
				}))
			},
		)
	} else {
		Err(Status::Unauthorized)
	}
}
