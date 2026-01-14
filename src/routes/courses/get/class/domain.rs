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
