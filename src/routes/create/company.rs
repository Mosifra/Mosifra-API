use rocket::{http::Status, serde::json::Json};

use crate::{
	models::{auth::AuthGuard, users::Company},
	postgres::Db,
	utils::mail::verify_mail,
};

use super::domain::{CreateCompanyPayload, CreateUserResponse};

#[utoipa::path(
	post,
	path = "/create/company",
	request_body = CreateCompanyPayload,
	responses(
		(status = 200, description = "Company created successfully", body = CreateUserResponse),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal Server Error")
	),
	security(
		("api_key" = [])
	),
	tag = "Create"
)]
#[post("/create/company", data = "<create_company_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_company(
	auth: AuthGuard,
	create_company_payload: Json<CreateCompanyPayload>,
) -> Result<Json<CreateUserResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_admin() {
		let company = Company::try_from(create_company_payload.into_inner())?;

		if !verify_mail(&company.mail)? {
			return Ok(Json(CreateUserResponse {
				success: false,
				password: None,
			}));
		}

		let is_inserted = company.insert().await;

		if is_inserted.is_ok() {
			Ok(Json(CreateUserResponse {
				success: true,
				password: Some(company.password),
			}))
		} else {
			Ok(Json(CreateUserResponse {
				success: false,
				password: None,
			}))
		}
	} else {
		Err(Status::Unauthorized)
	}
}
