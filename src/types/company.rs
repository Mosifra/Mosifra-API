use passwords::PasswordGenerator;
use uuid::Uuid;

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
	type Error = ();

	fn try_from(value: CompanyDto) -> Result<Self, Self::Error> {
		let password = PasswordGenerator::new()
			.length(8)
			.numbers(true)
			.lowercase_letters(true)
			.uppercase_letters(true)
			.symbols(true)
			.spaces(false)
			.exclude_similar_characters(true)
			.strict(true)
			.generate_one()
			.unwrap();

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
