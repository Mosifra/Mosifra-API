use std::ops::Index;

use rocket::{http::Status, serde::json::Json};

use crate::{
	error_handling::StatusOptionHandling,
	models::{auth::AuthGuard, courses::Internship},
};

use super::domain::{GetInternshipsPayload, GetInternshipsResponse};

#[utoipa::path(
	post,
	path = "/courses/internships",
	request_body = GetInternshipsPayload,
	responses(
		(status = 200, description = "List of internships", body = GetInternshipsResponse),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal Server Error")
	),
	security(
		("api_key" = [])
	),
	tag = "Courses"
)]
#[post("/courses/internships", data = "<get_internships_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_internships(
	auth: AuthGuard,
	get_internships_payload: Json<GetInternshipsPayload>,
) -> Result<Json<GetInternshipsResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;
	let payload = get_internships_payload.into_inner();

	if generic_user.is_university()
		&& let Some(course_types) = payload.course_types
	{
		let internships = Internship::get_all_based_on_course_types(course_types).await?;

		Ok(Json(GetInternshipsResponse {
			success: true,
			internships,
		}))
	} else if generic_user.is_student()
		&& let Some(course_types) = payload.course_types
		&& course_types.len() == 1
	{
		let course_type = course_types.index(0);
		let student = generic_user.to_student()?;
		let class = student
			.get_class()
			.await?
			.internal_server_error("Student has no class (Should not be possible)")?;
		if class.course_type == *course_type {
			let internships = Internship::get_all_based_on_course_types(course_types).await?;

			Ok(Json(GetInternshipsResponse {
				success: true,
				internships,
			}))
		} else {
			Err(Status::Unauthorized)
		}
	} else {
		Err(Status::Unauthorized)
	}
}
