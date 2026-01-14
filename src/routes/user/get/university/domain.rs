use serde::Serialize;

use crate::models::courses::CourseType;

// get course_types
#[derive(Debug, Serialize)]
pub struct GetCourseTypesResponse {
	pub success: bool,
	pub course_type: Vec<CourseType>,
}
