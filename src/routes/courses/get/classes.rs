use rocket::{http::Status, serde::json::Json};

use crate::models::auth::AuthGuard;

use super::domain::GetClassesResponse;

#[get("/courses/classes")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_classes(auth: AuthGuard) -> Result<Json<GetClassesResponse>, Status> {
	auth.get_classes().await
}
