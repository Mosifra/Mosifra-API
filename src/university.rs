use passwords::PasswordGenerator;
use uuid::Uuid;

use crate::{
	class::{Class, ClassDto, TryFromVecClassDtoToClassVec},
	internship::{Internship, InternshipDto, TryFromVecInternshipDtoToInternshipVec},
};

#[derive(Debug)]
pub struct University {
	pub id: String,
	pub login: String,
	pub password: String,
	pub name: String,
	pub mail: String,
	pub class_list: Vec<Class>,
	pub intership_list: Vec<Internship>,
}

#[derive(Debug, FromForm)]
pub struct UniversityDto {
	pub login: String,
	pub name: String,
	pub mail: String,
	pub class_list: Vec<ClassDto>,
	pub intership_list: Vec<InternshipDto>,
}

impl TryFrom<UniversityDto> for University {
	type Error = ();

	fn try_from(value: UniversityDto) -> Result<Self, Self::Error> {
		let class_list = Vec::<Class>::try_from_classdto_vec_to_class_vec(value.class_list)?;
		let intership_list =
			Vec::<Internship>::try_from_internshipdto_vec_to_internship_vec(value.intership_list)?;

		#[allow(clippy::unwrap_used)]
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
			name: value.name,
			mail: value.mail,
			class_list,
			intership_list,
		})
	}
}
