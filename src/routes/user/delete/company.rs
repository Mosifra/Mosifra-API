use rocket::{http::Status, serde::json::Json};

use crate::{
	models::{auth::AuthGuard, users::Company},
	postgres::Db,
};

use super::domain::{DeleteCompanyPayload, DeleteCompanyResponse};

#[utoipa::path(
	delete,
	path = "/user/company",
	request_body = DeleteCompanyPayload,
	responses(
		(status = 200, description = "Company deleted successfully", body = DeleteCompanyResponse),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal Server Error")
	),
	security(
		("api_key" = [])
	),
	tag = "User"
)]
#[delete("/user/company", data = "<delete_company_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn delete_company(
	delete_company_payload: Json<DeleteCompanyPayload>,
	auth: AuthGuard,
) -> Result<Json<DeleteCompanyResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_admin() {
		let company = Company::from_id(delete_company_payload.id.clone()).await?;
		Ok(Json(DeleteCompanyResponse {
			success: company.delete().await.is_ok(),
		}))
	} else {
		Err(Status::Unauthorized)
	}
}
