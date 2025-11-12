use rocket::{http::Status, serde::json::Json};

use crate::structs::jwt::UserJwt;

use super::domain::{CheckSessionPayload, CheckSessionResponse};

#[post("/check_session", data = "<check_session_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn check_session(
	check_session_payload: Json<CheckSessionPayload>,
) -> Result<Json<CheckSessionResponse>, Status> {
	let jwt = &check_session_payload.into_inner().jwt;
	let user_jwt = UserJwt::from_raw_jwt(jwt)?;
	let Some(_) = user_jwt else {
		return Ok(Json(CheckSessionResponse {
			valid: false,
			error: Some("Invalid session or incorrect user type".to_string()),
		}));
	};

	Ok(Json(CheckSessionResponse {
		valid: true,
		error: None,
	}))
}
