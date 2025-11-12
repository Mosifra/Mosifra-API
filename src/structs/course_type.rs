use std::str::FromStr;

use rocket::http::Status;

#[derive(Debug)]
pub enum CourseType {
	Info,
}

impl FromStr for CourseType {
	type Err = Status;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Info" => Ok(Self::Info),
			e => {
				eprintln!("Error trying to convert CourseType to string: {e}");
				Err(Status::InternalServerError)
			}
		}
	}
}
