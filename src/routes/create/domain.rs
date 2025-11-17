// Company

use chrono::NaiveDate;
use rocket::{fs::TempFile, http::Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	models::{
		courses::CourseType,
		users::{Company, University},
	},
	utils::crypto::generate_password,
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

#[derive(Debug, FromForm)]
pub struct StudentCsvPayload<'r> {
	pub csv: TempFile<'r>,
	pub class: String,
}

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

// Class

#[derive(Debug, Deserialize)]
pub struct CreateClassPayload {
	pub name: String,
	pub course_type: CourseType,
	pub date_internship_start: NaiveDate,
	pub date_internship_end: NaiveDate,
	pub maximum_internship_length: i32,
	pub minimum_internship_length: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateClassResponse {
	pub success: bool,
}
