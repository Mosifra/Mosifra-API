use std::str::FromStr;

use rocket::time::Date;
use uuid::Uuid;

use crate::course_type::CourseType;

use anyhow::Result;

#[derive(Debug)]
#[allow(dead_code)] // WIP
pub struct Internship {
	id: String,
	course_type: CourseType,
	date_start: Date,
	date_end: Date,
	#[allow(clippy::struct_field_names)] // Normal
	internship_duration_min_in_weeks: u8,
	#[allow(clippy::struct_field_names)] // Normal
	internship_duration_max_in_weeks: u8,
	title: String,
	description: String,
	place: String,
}

#[derive(Debug, FromForm)]
pub struct InternshipDto {
	course_type: String,
	date_start: Date,
	date_end: Date,
	internship_duration_min_in_weeks: u8,
	internship_duration_max_in_weeks: u8,
	title: String,
	description: String,
	place: String,
}

impl TryFrom<InternshipDto> for Internship {
	type Error = ();

	fn try_from(value: InternshipDto) -> Result<Self, Self::Error> {
		Ok(Self {
			id: Uuid::new_v4().to_string(),
			course_type: CourseType::from_str(&value.course_type)?,
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

pub trait TryFromVecInternshipDtoToInternshipVec {
	// Not sure how to fix this one
	#[allow(clippy::result_unit_err)]
	fn try_from_internshipdto_vec_to_internship_vec(value: Vec<InternshipDto>) -> Result<Self, ()>
	where
		Self: std::marker::Sized;
}

impl TryFromVecInternshipDtoToInternshipVec for Vec<Internship> {
	fn try_from_internshipdto_vec_to_internship_vec(value: Vec<InternshipDto>) -> Result<Self, ()> {
		let mut res = vec![];
		for class_dto in value {
			res.push(Internship::try_from(class_dto)?);
		}
		Ok(res)
	}
}
