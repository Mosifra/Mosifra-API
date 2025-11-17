use rocket::serde::json::Json;

use crate::models::auth::AuthGuard;

use super::domain::GetUserTypeResponse;

#[get("/user/user_type")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_user_type(auth: AuthGuard) -> Json<GetUserTypeResponse> {
	Json(GetUserTypeResponse {
		user_type: auth.user_type,
	})
}
