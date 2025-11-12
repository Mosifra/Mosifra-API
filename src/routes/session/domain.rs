use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CheckSessionPayload {
	pub jwt: String,
}

#[derive(Debug, Serialize)]
pub struct CheckSessionResponse {
	pub valid: bool,
	pub error: Option<String>,
}
