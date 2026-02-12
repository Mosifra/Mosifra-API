use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::courses::{CourseType, Internship, dto::class::ClassDto};

// Classes

#[derive(Debug, Serialize, ToSchema)]
pub struct GetClassesResponse {
	pub success: bool,
	pub classes: Option<Vec<ClassDto>>,
}

// Internships

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetInternshipsPayload {
	pub course_types: Option<Vec<CourseType>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetInternshipsResponse {
	pub success: bool,
	pub internships: Vec<Internship>,
}
