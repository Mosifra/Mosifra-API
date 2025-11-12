use rocket::form::Form;

use crate::{
	structs::company::{Company, CompanyDto},
	traits::db::Db,
	utils::verify_mail,
};

#[post("/user/create_company", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_company(form: Form<CompanyDto>) -> Result<String, String> {
	let company = Company::try_from(form.into_inner())?;
	println!("{company:#?}");

	if verify_mail(&company.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	company.insert().await
}
