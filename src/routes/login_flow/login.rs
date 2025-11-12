use std::str::FromStr;

use rocket::{http::Status, serde::json::Json};

use crate::{
	structs::{company::Company, student::Student, university::University, user_type::UserType},
	traits::db::Db,
	utils::set_transaction_id,
};

use super::domain::{LoginPayload, LoginResponse};

#[post("/login", data = "<login_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login(login_payload: Json<LoginPayload>) -> Result<Json<LoginResponse>, Status> {
	let login = login_payload.into_inner();
	let user_type = UserType::from_str(&login.user_type)?;

	match user_type {
		UserType::Admin => todo!(),
		UserType::University => login_university(login).await,
		UserType::Student => login_student(login).await,
		UserType::Company => login_company(login).await,
	}
}

pub async fn login_university(login: LoginPayload) -> Result<Json<LoginResponse>, Status> {
	let university = University::login(&login.login, &login.password).await?;

	match university {
		Some(university) => {
			set_transaction_id(&university.mail, &university.id, login.remember_me).await
		}
		None => Ok(Json(LoginResponse {
			valid: false,
			transaction_id: None,
			remember_me: None,
		})),
	}
}

pub async fn login_company(login: LoginPayload) -> Result<Json<LoginResponse>, Status> {
	let company = Company::login(&login.login, &login.password).await?;

	match company {
		Some(company) => set_transaction_id(&company.mail, &company.id, login.remember_me).await,
		None => Ok(Json(LoginResponse {
			valid: false,
			transaction_id: None,
			remember_me: None,
		})),
	}
}

pub async fn login_student(login: LoginPayload) -> Result<Json<LoginResponse>, Status> {
	let student = Student::login(&login.login, &login.password).await?;

	match student {
		Some(student) => set_transaction_id(&student.mail, &student.id, login.remember_me).await,
		None => Ok(Json(LoginResponse {
			valid: false,
			transaction_id: None,
			remember_me: None,
		})),
	}
}
