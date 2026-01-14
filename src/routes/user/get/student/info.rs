use rocket::{http::Status, serde::json::Json};

use crate::{error_handling::StatusOptionHandling, models::auth::AuthGuard};

use super::domain::GetInfoPayload;

#[get("/user/student/info")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_student_info(auth: AuthGuard) -> Result<Json<GetInfoPayload>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_student() {
		let student = generic_user.to_student()?;
		Ok(Json(GetInfoPayload {
			success: true,
			first_name: Some(student.first_name.clone()),
			last_name: Some(student.last_name.clone()),
			email: Some(student.mail.clone()),
			university: Some(student.get_university().await?.name),
			class_name: Some(
				student
					.get_class()
					.await?
					.internal_server_error("Student exists but has no class")?
					.name,
			),
		}))
	} else {
		Err(Status::Unauthorized)
	}
}
