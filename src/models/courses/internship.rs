use chrono::NaiveDate;
use rocket::http::Status;
use serde::Deserialize;
use uuid::Uuid;

use anyhow::Result;

use super::course_type::CourseType;

#[derive(Debug)]
#[allow(dead_code)] // WIP
pub struct Internship {
	id: String,
	course_type: CourseType,
	date_start: NaiveDate,
	date_end: NaiveDate,
	#[allow(clippy::struct_field_names)] // Normal
	internship_duration_min_in_weeks: u8,
	#[allow(clippy::struct_field_names)] // Normal
	internship_duration_max_in_weeks: u8,
	title: String,
	description: String,
	place: String,
}

#[derive(Debug, Deserialize)]
pub struct InternshipDto {
	course_type: i32,
	date_start: NaiveDate,
	date_end: NaiveDate,
	internship_duration_min_in_weeks: u8,
	internship_duration_max_in_weeks: u8,
	title: String,
	description: String,
	place: String,
}

impl TryFrom<InternshipDto> for Internship {
	type Error = Status;

	fn try_from(value: InternshipDto) -> Result<Self, Self::Error> {
		Ok(Self {
			id: Uuid::new_v4().to_string(),
			course_type: CourseType::from_sql(value.course_type)?,
			date_start: value.date_start,
			date_end: value.date_end,
			internship_duration_min_in_weeks: value.internship_duration_min_in_weeks,
			internship_duration_max_in_weeks: value.internship_duration_max_in_weeks,
			title: value.title,
			description: value.description,
			place: value.place,
		})
	}
}
