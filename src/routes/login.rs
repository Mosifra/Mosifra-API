use rocket::form::Form;

use crate::{
	redis,
	structs::{company::Company, student::Student, university::University},
	traits::db::Db,
	utils::{send_2fa_mail, verify_mail, verify_password},
};

#[derive(Debug, FromForm)]
pub struct Login {
	mail: String,
	password: String,
}

#[derive(Debug, FromForm)]
pub struct Twofa {
	pub code: String,
	pub transaction_id: String,
}

#[post("/login_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_university(form: Form<Login>) -> Result<String, String> {
	let login = form.into_inner();

	if !verify_mail(&login.mail) {
		return Err("Incorrect Mail".to_string());
	}

	let correct_password = University::get_password_from_mail(&login.mail).await?;
	let id = University::get_id_from_mail(&login.mail).await?;

	if verify_password(&login.password, &correct_password)? {
		let code = send_2fa_mail(&login.mail).await?;
		let transaction_id = redis::get_transactionid(id, code)?;
		Ok(format!("{{\"transaction_id\":\"{transaction_id}\"}}"))
	} else {
		Err("Invalid Password".to_string())
	}
}

#[post("/login_company", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_company(form: Form<Login>) -> Result<String, String> {
	let login = form.into_inner();

	if !verify_mail(&login.mail) {
		return Err("Incorrect Mail".to_string());
	}

	let correct_password = Company::get_password_from_mail(&login.mail).await?;

	if verify_password(&login.password, &correct_password)? {
		Ok("Logged in".to_string())
	} else {
		Err("Invalid Password".to_string())
	}
}

#[post("/login_student", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_student(form: Form<Login>) -> Result<String, String> {
	let login = form.into_inner();

	if !verify_mail(&login.mail) {
		return Err("Incorrect Mail".to_string());
	}

	let correct_password = Student::get_password_from_mail(&login.mail).await?;

	if verify_password(&login.password, &correct_password)? {
		Ok("Logged in".to_string())
	} else {
		Err("Invalid Password".to_string())
	}
}

#[post("/2fa", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn twofa(form: Form<Twofa>) -> Result<String, String> {
	let twofa = form.into_inner();

	if redis::check_2fa_code(&twofa)? {
		redis::invalidate_transactionid(&twofa)?;
		Ok("Logged in".to_string())
	} else {
		Ok("Incorrect code".to_string())
	}
}
