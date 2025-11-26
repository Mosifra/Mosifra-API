use std::str::FromStr;

use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CourseType {
	Info,
}

pub const INFO_DB: i32 = 1;

impl CourseType {
	#[must_use]
	pub const fn to_sql(&self) -> i32 {
		match self {
			Self::Info => INFO_DB,
		}
	}

	pub const fn from_sql(course_type_id: i32) -> Result<Self, Status> {
		match course_type_id {
			INFO_DB => Ok(Self::Info),
			_ => Err(Status::InternalServerError),
		}
	}
}
