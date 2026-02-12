use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteClassPayload {
	pub class_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteClassResponse {
	pub success: bool,
}
