use rocket::time::Date;

use crate::course_type::CourseType;

#[derive(Debug)]
pub struct Internship {
	id: String,
	course_type: CourseType,
	date_start: Date,
	date_end: Date,
	internship_duration_min_in_weeks: u8,
	internship_duration_max_in_weeks: u8,
	title: String,
	description: String,
	place: String,
}

#[derive(Debug, FromForm)]
pub struct InternshipDto {
	id: String,
	course_type: String,
	date_start: Date,
	date_end: Date,
	internship_duration_min_in_weeks: u8,
	internship_duration_max_in_weeks: u8,
	title: String,
	description: String,
	place: String,
}
