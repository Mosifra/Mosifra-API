use rocket::{http::Status, serde::json::Json};

use crate::models::auth::AuthGuard;

use super::domain::GetCourseTypeResponse;

#[get("/user/student/course_type")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_student_course_type(
	auth: AuthGuard,
) -> Result<Json<GetCourseTypeResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_student() {
		let student = generic_user.to_student()?;
		Ok(Json(GetCourseTypeResponse {
			success: true,
			course_type: Some(vec![student.get_course_type().await?]),
		}))
	} else {
		Err(Status::Unauthorized)
	}
}
