use serde::Serialize;
use utoipa::ToSchema;

use crate::models::courses::CourseType;

// get_info

#[derive(Debug, Serialize, ToSchema)]
pub struct GetInfoResponse {
	pub success: bool,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub email: Option<String>,
	pub university: Option<String>,
	pub class_name: Option<String>,
}

// get course type

#[derive(Debug, Serialize, ToSchema)]
pub struct GetCourseTypeResponse {
	pub success: bool,
	pub course_type: Option<Vec<CourseType>>,
}
