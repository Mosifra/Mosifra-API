use rocket::{http::Status, serde::json::Json};

use crate::models::auth::AuthGuard;

use super::domain::GetCourseTypesResponse;

#[get("/user/university/course_types")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_university_course_types(
	auth: AuthGuard,
) -> Result<Json<GetCourseTypesResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_university() {
		let university = generic_user.to_university()?;
		let course_types = university.get_course_types().await?;

		Ok(Json(GetCourseTypesResponse {
			success: true,
			course_type: course_types, // Bad but more useful for the front
		}))
	} else {
		Err(Status::Unauthorized)
	}
}
