use rocket::form::Form;
use uuid::Uuid;

use crate::{
	redis::{self, get_user_id_from_twofa},
	structs::{company::Company, student::Student, university::University},
	traits::db::Db,
	utils::{send_2fa_mail, verify_mail, verify_password},
};

use serde_json::json;

#[derive(Debug, FromForm)]
pub struct Login {
	mail: String,
	password: String,
	remember_me: bool,
}

#[derive(Debug, FromForm)]
pub struct Twofa {
	pub code: String,
	pub transaction_id: String,
	pub user_type: String,
	pub remember_me: bool,
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
		Ok(format!(
			"{{\"transaction_id\":\"{transaction_id}\",\"remember_me\":{}}}",
			login.remember_me
		))
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

#[post("/twofa", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn twofa(form: Form<Twofa>) -> Result<String, String> {
	let twofa = form.into_inner();

	if redis::check_2fa_code(&twofa)? {
		let session_id = Uuid::new_v4().to_string();
		let session_data = json!({
			"user_id": get_user_id_from_twofa(&twofa),
			"user_type": twofa.user_type,
		});

		let ttl_seconds: u64 = if twofa.remember_me {
			30 * 24 * 3600
		} else {
			30 * 60
		};
		println!("{session_id}, {session_data}, {ttl_seconds}");
		redis::set_session(&session_id, &session_data, ttl_seconds)?;
		redis::invalidate_transactionid(&twofa)?;

		Ok("Logged in".to_string())
	} else {
		Ok("Incorrect code".to_string())
	}
}
