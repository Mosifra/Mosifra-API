use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Login

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
	pub login: String,
	pub password: String,
	pub remember_me: bool,
	pub user_type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
	pub valid: bool,
	pub transaction_id: Option<String>,
	pub remember_me: Option<bool>,
}

// Twofa

#[derive(Debug, Deserialize, ToSchema)]
pub struct TwofaPayload {
	pub code: String,
	pub transaction_id: String,
	pub user_type: String,
	pub remember_me: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TwofaResponse {
	pub valid: bool,
	pub jwt: Option<String>,
}

// CheckSession

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckSessionResponse {
	pub valid: bool,
	pub error: Option<String>,
}

// Disconnect

#[derive(Debug, Serialize, ToSchema)]
pub struct DisconnectResponse {
	pub success: bool,
}
