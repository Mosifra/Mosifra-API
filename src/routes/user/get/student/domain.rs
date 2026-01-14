use serde::Serialize;

use crate::models::courses::CourseType;

// get_info

#[derive(Debug, Serialize)]
pub struct GetInfoPayload {
	pub success: bool,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub email: Option<String>,
	pub university: Option<String>,
	pub class_name: Option<String>,
}

// get course type

#[derive(Debug, Serialize)]
pub struct GetCourseTypePayload {
	pub success: bool,
	pub course_type: Option<CourseType>,
}
