use rocket::{http::Status, serde::json::Json};

use crate::models::auth::AuthGuard;

use super::domain::GetInfoResponse;

#[get("/user/student_info")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_student_info(auth: AuthGuard) -> Result<Json<GetInfoResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_student() {
		let student = generic_user.to_student()?;
		student.get_info().await
	} else {
		Err(Status::Unauthorized)
	}
}
