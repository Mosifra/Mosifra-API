use serde::{Deserialize, Serialize};

use crate::models::{
	auth::UserType,
	users::{Company, University},
};

// user_type

#[derive(Debug, Serialize)]
pub struct GetUserTypeResponse {
	pub user_type: UserType,
}

// Universities
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniversitiesResponse {
	pub success: bool,
	pub universities: Option<Vec<University>>,
}

// Companies
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompaniesResponse {
	pub success: bool,
	pub companies: Option<Vec<Company>>,
}
