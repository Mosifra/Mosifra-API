use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::users::dto::StudentDto;

// Get Students of class

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetClassStudentsPayload {
	pub class_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetClassStudentsResponse {
	pub success: bool,
	pub students: Option<Vec<StudentDto>>,
}
