use rocket::{http::Status, serde::json::Json};

use crate::models::auth::AuthGuard;

use super::domain::DisconnectResponse;

#[utoipa::path(
	delete,
	path = "/auth/logout",
	responses(
		(status = 200, description = "Logout successful", body = DisconnectResponse),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal Server Error")
	),
	security(
		("api_key" = [])
	),
	tag = "Auth"
)]
#[delete("/auth/logout")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn logout(auth: AuthGuard) -> Result<Json<DisconnectResponse>, Status> {
	auth.get_generic_user().await?.logout()
}
