use rocket::{http::Status, serde::json::Json};

use crate::{
	error_handling::StatusOptionHandling,
	models::{auth::AuthGuard, courses::Class},
};

use super::domain::{GetCourseTypeFromClassIdPayload, GetCourseTypeFromClassIdResponse};

#[post(
	"/courses/class/course_type",
	data = "<get_course_type_from_class_id_payload>"
)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_class_course_type(
	auth: AuthGuard,
	get_course_type_from_class_id_payload: Json<GetCourseTypeFromClassIdPayload>,
) -> Result<Json<GetCourseTypeFromClassIdResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;
	let class_id = &get_course_type_from_class_id_payload.class_id;
	if generic_user.is_university() {
		let university = generic_user.to_university()?;
		if university.has_class(class_id) {
			let class = Class::from_id(class_id.to_string())
				.await?
				.internal_server_error("University has class but class does not exist")?;
			Ok(Json(GetCourseTypeFromClassIdResponse {
				success: true,
				course_type: Some(class.course_type),
			}))
		} else {
			Err(Status::Unauthorized)
		}
	} else if generic_user.is_student() {
		let student = generic_user.to_student()?;
		if student.is_in_class(class_id).await? {
			let class = Class::from_id(class_id.to_string())
				.await?
				.internal_server_error("Student is in a class that does not exist")?;
			Ok(Json(GetCourseTypeFromClassIdResponse {
				success: true,
				course_type: Some(class.course_type),
			}))
		} else {
			Err(Status::Unauthorized)
		}
	} else {
		Err(Status::Unauthorized)
	}
}
