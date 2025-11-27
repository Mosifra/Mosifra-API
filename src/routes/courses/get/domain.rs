use serde::{Deserialize, Serialize};

use crate::models::courses::{CourseType, Internship, dto::class::ClassDto};

// Classes

#[derive(Debug, Serialize)]
pub struct GetClassesResponse {
	pub success: bool,
	pub classes: Option<Vec<ClassDto>>,
}

// Internships

#[derive(Debug, Deserialize)]
pub struct GetInternshipsPayload {
	pub course_type: Option<CourseType>,
}

#[derive(Debug, Serialize)]
pub struct GetInternshipsResponse {
	pub success: bool,
	pub internships: Vec<Internship>,
}
