use serde::Serialize;
use utoipa::ToSchema;

use crate::models::courses::CourseType;

// get course_types
#[derive(Debug, Serialize, ToSchema)]
pub struct GetCourseTypesResponse {
	pub success: bool,
	pub course_type: Vec<CourseType>,
}
