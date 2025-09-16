use rocket::time::Date;

use crate::course_type::CourseType;

#[derive(Debug)]
pub struct Class {
	id: String,
	name: String,
	course_type: CourseType,
	date_internship_start: Date,
	date_internship_end: Date,
}

#[derive(Debug, FromForm)]
pub struct ClassDto {
	id: String,
	name: String,
	course_type: String,
	date_internship_start: Date,
	date_internship_end: Date,
}
