use serde::{Deserialize, Serialize};

use crate::models::{
    courses::{CourseType, Internship, dto::class::ClassDto},
    users::University,
};

// Classes

#[derive(Debug, Serialize)]
pub struct GetClassesResponse {
    pub success: bool,
    pub classes: Option<Vec<ClassDto>>,
}

// Internships

#[derive(Debug, Deserialize)]
pub struct GetInternshipsPayload {
    pub course_types: Option<Vec<CourseType>>,
}

#[derive(Debug, Serialize)]
pub struct GetInternshipsResponse {
    pub success: bool,
    pub internships: Vec<Internship>,
}

// Universities
#[derive(Serialize, Deserialize)]
pub struct GetUniversitiesResponse {
    pub success: bool,
    pub universities: Vec<University>,
}
