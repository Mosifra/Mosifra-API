use serde::{Deserialize, Serialize};

use crate::models::{courses::CourseType, users::dto::StudentDto};

// Get Students of class

#[derive(Debug, Deserialize)]
pub struct GetClassStudentsPayload {
	pub class_id: String,
}

#[derive(Debug, Serialize)]
pub struct GetClassStudentsResponse {
	pub success: bool,
	pub students: Option<Vec<StudentDto>>,
}

// Get Course Type from Class Id

#[derive(Debug, Deserialize)]
pub struct GetCourseTypeFromClassIdPayload {
	pub class_id: String,
}

#[derive(Debug, Serialize)]
pub struct GetCourseTypeFromClassIdResponse {
	pub success: bool,
	pub course_type: Option<CourseType>,
}
