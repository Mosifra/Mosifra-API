// Company

use rocket::http::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	structs::{company::Company, university::University},
	utils::generate_password,
};

#[derive(Debug, Deserialize)]
pub struct CreateCompanyPayload {
	pub login: String,
	pub mail: String,
	pub name: String,
}

impl TryFrom<CreateCompanyPayload> for Company {
	type Error = Status;

	fn try_from(value: CreateCompanyPayload) -> Result<Self, Self::Error> {
		let password = generate_password()?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			login: value.login,
			password,
			mail: value.mail,
			name: value.name,
			internship_list: Vec::new(),
		})
	}
}

#[derive(Debug, Serialize)]
pub struct CreateCompanyResponse {
	pub success: bool,
}

// Student

#[derive(Debug, Serialize)]
pub struct StudentCsvResponse {
	pub success: bool,
}

// University

#[derive(Debug, Deserialize)]
pub struct CreateUniversityPayload {
	pub login: String,
	pub name: String,
	pub mail: String,
}

impl TryFrom<CreateUniversityPayload> for University {
	type Error = Status;

	fn try_from(value: CreateUniversityPayload) -> Result<Self, Self::Error> {
		let password = generate_password()?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			login: value.login,
			password,
			name: value.name,
			mail: value.mail,
			class_list: vec![],
			intership_list: vec![],
		})
	}
}

#[derive(Debug, Serialize)]
pub struct CreateUniversityResponse {
	pub success: bool,
}
