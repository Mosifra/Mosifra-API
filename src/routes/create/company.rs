use rocket::{http::Status, serde::json::Json};

use crate::{models::users::Company, postgres::Db, utils::mail::verify_mail};

use super::domain::{CreateCompanyPayload, CreateCompanyResponse};

#[post("/create/company", data = "<create_company_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_company(
	create_company_payload: Json<CreateCompanyPayload>,
) -> Result<Json<CreateCompanyResponse>, Status> {
	let company = Company::try_from(create_company_payload.into_inner())?;
	println!("{company:#?}");

	if verify_mail(&company.mail)? {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	let is_done = company.insert().await;

	Ok(Json(CreateCompanyResponse {
		success: is_done.is_ok(),
	}))
}
