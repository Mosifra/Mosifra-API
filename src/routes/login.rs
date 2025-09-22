use rocket::form::Form;

use crate::{
	db::get_university_password_from_mail,
	utils::{verify_mail, verify_password},
};

#[derive(Debug, FromForm)]
pub struct Login {
	mail: String,
	password: String,
}

#[post("/login_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_university(form: Form<Login>) -> Result<String, String> {
	let login = form.into_inner();

	if !verify_mail(&login.mail) {
		return Err("Incorrect Mail".to_string());
	}

	let correct_password = get_university_password_from_mail(&login.mail).await?;

	if verify_password(&login.password, &correct_password)? {
		Ok("Logged in".to_string())
	} else {
		Err("Invalid Password".to_string())
	}
}
