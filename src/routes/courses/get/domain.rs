use serde::Serialize;

use crate::models::courses::Class;

// Classes

#[derive(Debug, Serialize)]
pub struct GetClassesResponse {
	pub success: bool,
	pub classes: Option<Vec<Class>>,
}
