use rocket::form::Form;

use crate::{
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
	twofa: String,
	current_id: String,
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
		University::insert_2fa(&id, &code);
		Ok("Sent mail for 2fa".to_string())
	} else {
		Err("Invalid Password".to_string())
	}
}

// #[post("/login_university/2fa", data = "<form>")]
// #[allow(clippy::needless_pass_by_value)]
// #[allow(clippy::missing_errors_doc)]
// pub async fn twofa_university(form: Form<Twofa>) -> Result<String, String> {
// 	let twofa_form = form.into_inner();
// 	let id = twofa_form.current_id;
// 	let twofa = twofa_form.twofa;

// 	let correct_password = University::from_id(&id).await?;

// 	if verify_password(&twofa.password, &correct_password)? {
// 		let code = send_2fa_mail(&twofa.mail).await?;
// 		University::insert_2fa(&id, &code);
// 		Ok("Sent mail for 2fa".to_string())
// 	} else {
// 		Err("Invalid Password".to_string())
// 	}
// }

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
