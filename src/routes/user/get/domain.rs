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
