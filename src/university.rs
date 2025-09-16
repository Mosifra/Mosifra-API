use crate::{
	class::{Class, ClassDto},
	internship::{Internship, InternshipDto},
};

#[derive(Debug)]
pub struct University {
	id: String,
	login: String,
	password: String,
	name: String,
	mail: String,
	class_list: Vec<Class>,
	intership_list: Vec<Internship>,
}

#[derive(Debug, FromForm)]
pub struct UniversityDto {
	pub id: String,
	pub login: String,
	pub password: String,
	pub name: String,
	pub mail: String,
	pub class_list: Vec<ClassDto>,
	pub intership_list: Vec<InternshipDto>,
}
