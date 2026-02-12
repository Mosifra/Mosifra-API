use rocket::{http::Status, serde::json::Json};

use crate::models::{auth::AuthGuard, users::Company};

use super::domain::GetCompaniesResponse;

#[utoipa::path(
	get,
	path = "/user/companies",
	responses(
		(status = 200, description = "List of companies", body = GetCompaniesResponse),
		(status = 401, description = "Unauthorized")
	),
	security(
		("api_key" = [])
	),
	tag = "User"
)]
#[get("/user/companies")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_companies(auth: AuthGuard) -> Result<Json<GetCompaniesResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;
	if generic_user.is_admin() {
		let companies = Company::get_all().await;

		companies.map_or(
			Ok(Json(GetCompaniesResponse {
				success: false,
				companies: None,
			})),
			|companies| {
				Ok(Json(GetCompaniesResponse {
					success: true,
					companies: Some(companies),
				}))
			},
		)
	} else {
		Err(Status::Unauthorized)
	}
}
