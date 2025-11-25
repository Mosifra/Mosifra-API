use rocket::{http::Status, serde::json::Json};

use crate::models::auth::AuthGuard;

use super::domain::DisconnectResponse;

#[delete("/auth/logout")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn logout(auth: AuthGuard) -> Result<Json<DisconnectResponse>, Status> {
	auth.get_generic_user().await?.logout()
}
