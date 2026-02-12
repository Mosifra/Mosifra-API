use rocket::serde::json::Json;

use crate::models::auth::AuthGuard;

use super::domain::GetUserTypeResponse;

#[utoipa::path(
	get,
	path = "/user/user_type",
	responses(
		(status = 200, description = "Get user type", body = GetUserTypeResponse),
		(status = 401, description = "Unauthorized")
	),
	security(
		("api_key" = [])
	),
	tag = "User"
)]
#[get("/user/user_type")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_user_type(auth: AuthGuard) -> Json<GetUserTypeResponse> {
	Json(GetUserTypeResponse {
		user_type: auth.user_type,
	})
}
