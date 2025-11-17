use serde::{Deserialize, Serialize};

use crate::models::courses::Class;

// Classes

#[derive(Debug, Deserialize)]
pub struct GetClassesPayload {
    pub jwt: String,
}

#[derive(Debug, Serialize)]
pub struct GetClassesResponse {
    pub success: bool,
    pub classes: Option<Vec<Class>>,
}