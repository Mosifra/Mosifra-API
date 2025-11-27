use rocket::{http::Status, serde::json::Json};

use crate::{
	error_handling::StatusOptionHandling,
	models::{auth::AuthGuard, courses::Internship},
};

use super::domain::{GetInternshipsPayload, GetInternshipsResponse};

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
		&& let Some(course_type) = payload.course_type
	{
		let internships = Internship::get_all_based_on_course_type(course_type).await?;

		Ok(Json(GetInternshipsResponse {
			success: true,
			internships,
		}))
	} else if generic_user.is_university() && payload.course_type.is_none() {
		let internships = Internship::get_all().await?;

		Ok(Json(GetInternshipsResponse {
			success: true,
			internships,
		}))
	} else if generic_user.is_student()
		&& let Some(course_type) = payload.course_type
	{
		let student = generic_user.to_student()?;
		let class = student
			.get_class()
			.await?
			.internal_server_error("Student has no class (Should not be possible)")?;
		if class.course_type == course_type {
			let internships = Internship::get_all_based_on_course_type(course_type).await?;

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
