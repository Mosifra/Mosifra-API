use serde::Serialize;

use crate::models::courses::dto::class::ClassDto;

// Classes

#[derive(Debug, Serialize)]
pub struct GetClassesResponse {
	pub success: bool,
	pub classes: Option<Vec<ClassDto>>,
}
