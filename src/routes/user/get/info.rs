use rocket::{http::Status, serde::json::Json};

use crate::models::auth::UserJwt;

use super::domain::{GetInfoPayload, GetInfoResponse};

#[post("/user/student_info", data = "<get_student_info_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_student_info(
	get_student_info_payload: Json<GetInfoPayload>,
) -> Result<Json<GetInfoResponse>, Status> {
	let raw_jwt = get_student_info_payload.into_inner().jwt;

	let user_jwt = UserJwt::from_raw_jwt(&raw_jwt)?;
	match user_jwt {
		Some(user_jwt) => user_jwt.get_student_info().await,
		None => Ok(Json(GetInfoResponse {
			success: false,
			first_name: None,
			last_name: None,
			email: None,
			university: None,
			class_name: None,
		})),
	}
}
