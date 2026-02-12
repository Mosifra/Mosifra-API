use rocket::http::Status;
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::users::Student;

#[derive(Debug, Serialize, ToSchema)]
pub struct StudentDto {
	pub mail: String,
	pub first_name: String,
	pub last_name: String,
}

impl StudentDto {
	pub async fn from_id(id: String) -> Result<Self, Status> {
		let student = Student::from_id(id).await?;

		Ok(Self {
			mail: student.mail,
			first_name: student.first_name,
			last_name: student.last_name,
		})
	}
}
