use std::str::FromStr;

use rocket::form::Form;
use serde_json::json;

use crate::{
	structs::{
		company::Company, jwt::UserJwt, student::Student, university::University,
		user_type::UserType,
	},
	traits::db::Db,
	utils::set_transaction_id,
};

use crate::redis;

use super::domain::{Jwt, Login};

#[post("/login", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login(form: Form<Login>) -> Result<String, String> {
	let login = form.into_inner();
	let user_type = UserType::from_str(&login.user_type)?;

	match user_type {
		UserType::Admin => todo!(),
		UserType::University => login_university(login).await,
		UserType::Student => login_student(login).await,
		UserType::Company => login_company(login).await,
	}
}

pub async fn login_university(login: Login) -> Result<String, String> {
	let university = University::login(&login.login, &login.password).await;

	match university {
		Ok(university) => {
			set_transaction_id(&university.mail, &university.id, login.remember_me).await
		}
		Err(e) => Err(format!("Invalid Password: {e}")),
	}
}

pub async fn login_company(login: Login) -> Result<String, String> {
	let company = Company::login(&login.login, &login.password).await;

	match company {
		Ok(company) => set_transaction_id(&company.mail, &company.id, login.remember_me).await,
		Err(e) => Err(format!("Invalid Password: {e}")),
	}
}

pub async fn login_student(login: Login) -> Result<String, String> {
	let student = Student::login(&login.login, &login.password).await;

	match student {
		Ok(student) => set_transaction_id(&student.mail, &student.id, login.remember_me).await,
		Err(e) => Err(format!("Invalid Password: {e}")),
	}
}
