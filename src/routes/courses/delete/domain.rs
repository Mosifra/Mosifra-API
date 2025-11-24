use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DeleteClassPayload {
	pub class_id: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteClassResponse {
	pub success: bool,
}
