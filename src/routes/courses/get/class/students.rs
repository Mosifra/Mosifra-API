use rocket::{http::Status, serde::json::Json};

use crate::models::auth::AuthGuard;

use super::domain::{GetClassStudentsPayload, GetClassStudentsResponse};

#[post("/courses/class/students", data = "<get_class_students_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_class_students(
	auth: AuthGuard,
	get_class_students_payload: Json<GetClassStudentsPayload>,
) -> Result<Json<GetClassStudentsResponse>, Status> {
	let get_class_students_payload = get_class_students_payload.into_inner();
	auth.get_students_from_class_id(get_class_students_payload.class_id)
		.await
}
