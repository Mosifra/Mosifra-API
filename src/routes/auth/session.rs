use rocket::serde::json::Json;

use crate::models::auth::AuthGuard;

use super::domain::CheckSessionResponse;

#[get("/auth/check_session")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn check_session(_auth: AuthGuard) -> Json<CheckSessionResponse> {
	Json(CheckSessionResponse {
		valid: true,
		error: None,
	})
}
