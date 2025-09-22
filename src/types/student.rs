use uuid::Uuid;

use crate::utils::generate_password;

#[derive(Debug)]
pub struct Student {
	pub id: String,
	pub login: String,
	pub password: String,
	pub mail: String,
	pub first_name: String,
	pub last_name: String,
}

#[derive(Debug, FromForm)]
pub struct StudentDto {
	pub login: String,
	pub mail: String,
	pub first_name: String,
	pub last_name: String,
}

impl TryFrom<StudentDto> for Student {
	type Error = String;

	fn try_from(value: StudentDto) -> Result<Self, Self::Error> {
		let password = generate_password()?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			login: value.login,
			password,
			mail: value.mail,
			first_name: value.first_name,
			last_name: value.last_name,
		})
	}
}
