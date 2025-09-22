use uuid::Uuid;

use crate::utils::generate_password;

use super::internship::Internship;

#[derive(Debug)]
pub struct Company {
	pub id: String,
	pub login: String,
	pub password: String,
	pub mail: String,
	pub name: String,
	pub internship_list: Vec<Internship>,
}

#[derive(Debug, FromForm)]
pub struct CompanyDto {
	pub login: String,
	pub mail: String,
	pub name: String,
}

impl TryFrom<CompanyDto> for Company {
	type Error = String;

	fn try_from(value: CompanyDto) -> Result<Self, Self::Error> {
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
