use serde::Serialize;

use crate::models::auth::UserType;

// user_type

#[derive(Debug, Serialize)]
pub struct GetUserTypeResponse {
	pub user_type: UserType,
}
