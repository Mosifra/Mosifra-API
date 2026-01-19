use serde::{Deserialize, Serialize};

// Company

#[derive(Debug, Serialize)]
pub struct DeleteCompanyResponse {
	pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCompanyPayload {
	pub id: String,
}

// University

#[derive(Debug, Serialize)]
pub struct DeleteUniversityResponse {
	pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUniversityPayload {
	pub id: String,
}
