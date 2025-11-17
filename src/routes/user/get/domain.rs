use serde::{Deserialize, Serialize};

use crate::models::auth::UserType;

// user_type

#[derive(Debug, Deserialize)]
pub struct GetUserTypePayload {
	pub jwt: String,
}

#[derive(Debug, Serialize)]
pub struct GetUserTypeResponse {
	pub user_type: Option<UserType>,
}

// get_info

#[derive(Debug, Deserialize)]
pub struct GetInfoPayload {
	pub jwt: String,
}

#[derive(Debug, Serialize)]
pub struct GetInfoResponse {
	pub success: bool,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub email: Option<String>,
	pub university: Option<String>,
	pub class_name: Option<String>,
}
