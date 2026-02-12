use rocket::{http::Status, serde::json::Json};

use crate::{
	error_handling::StatusOptionHandling,
	models::{auth::AuthGuard, courses::Class},
};

use super::domain::{GetClassStudentsPayload, GetClassStudentsResponse};

#[utoipa::path(
	post,
	path = "/courses/class/students",
	request_body = GetClassStudentsPayload,
	responses(
		(status = 200, description = "List of students in class", body = GetClassStudentsResponse),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal Server Error")
	),
	security(
		("api_key" = [])
	),
	tag = "Courses"
)]
#[post("/courses/class/students", data = "<get_class_students_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_class_students(
	auth: AuthGuard,
	get_class_students_payload: Json<GetClassStudentsPayload>,
) -> Result<Json<GetClassStudentsResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;
	if auth.get_generic_user().await?.is_university() {
		let university = generic_user.to_university()?;
		let class_id = get_class_students_payload.class_id.clone();
		if university.has_class(&class_id) {
			let class = Class::from_id(class_id)
				.await?
				.internal_server_error("No classes for this id (Not possible ?)")?;

			let students = class.get_students().await?;

			Ok(Json(GetClassStudentsResponse {
				success: true,
				students: Some(students),
			}))
		} else {
			Err(Status::Unauthorized)
		}
	} else {
		Err(Status::Unauthorized)
	}
}
