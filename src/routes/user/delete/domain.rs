use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Company

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteCompanyResponse {
	pub success: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteCompanyPayload {
	pub id: String,
}

// University

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteUniversityResponse {
	pub success: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteUniversityPayload {
	pub id: String,
}
