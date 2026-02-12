// Company

use chrono::NaiveDate;
use rocket::{fs::TempFile, http::Status};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
	models::{
		courses::CourseType,
		users::{Company, University},
	},
	utils::crypto::generate_password,
};

#[derive(Debug, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUserResponse {
	pub success: bool,
	pub password: Option<String>,
}

// Student

#[derive(Debug, FromForm, ToSchema)]
pub struct StudentCsvPayload<'r> {
	#[schema(value_type = String, format = Binary)]
	pub csv: TempFile<'r>,
	pub class: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StudentCsvResponse {
	pub success: bool,
}

// University

#[derive(Debug, Deserialize, ToSchema)]
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

// Class

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateClassPayload {
	pub name: String,
	pub course_type: CourseType,
	pub date_internship_start: NaiveDate,
	pub date_internship_end: NaiveDate,
	pub maximum_internship_length: i32,
	pub minimum_internship_length: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateClassResponse {
	pub success: bool,
}

// Internship
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateIntershipPayload {
	pub course_type: CourseType,
	pub start_date: NaiveDate,
	pub end_date: NaiveDate,
	pub min_internship_length: i32,
	pub max_internship_length: i32,
	pub title: String,
	pub description: String,
	pub place: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateInternshipResponse {
	pub success: bool,
}
