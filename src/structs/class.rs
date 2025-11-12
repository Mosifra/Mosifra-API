use std::str::FromStr;

use chrono::NaiveDate;
use rocket::http::Status;
use serde::Deserialize;
use uuid::Uuid;

use super::course_type::CourseType;

// For now
#[allow(dead_code)]
#[derive(Debug)]
pub struct Class {
	id: String,
	name: String,
	course_type: CourseType,
	date_internship_start: NaiveDate,
	date_internship_end: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct ClassDto {
	name: String,
	course_type: String,
	date_internship_start: NaiveDate,
	date_internship_end: NaiveDate,
}

impl TryFrom<ClassDto> for Class {
	type Error = Status;

	fn try_from(value: ClassDto) -> Result<Self, Self::Error> {
		Ok(Self {
			id: Uuid::new_v4().to_string(),
			name: value.name,
			course_type: CourseType::from_str(&value.course_type)?,
			date_internship_start: value.date_internship_start,
			date_internship_end: value.date_internship_end,
		})
	}
}
