use std::str::FromStr;

use rocket::http::Status;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CourseType {
	Info,
}

pub const INFO_DB: i32 = 1;

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

impl CourseType {
	#[must_use]
	pub const fn to_sql(&self) -> i32 {
		match self {
			Self::Info => INFO_DB,
		}
	}
}
