use std::{fmt::Display, str::FromStr};

use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
	Admin,
	University,
	Student,
	Company,
}

impl Display for UserType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Admin => write!(f, "admin"),
			Self::University => write!(f, "university"),
			Self::Student => write!(f, "student"),
			Self::Company => write!(f, "company"),
		}
	}
}

impl FromStr for UserType {
	type Err = Status;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match value {
			"admin" => Ok(Self::Admin),
			"university" => Ok(Self::University),
			"student" => Ok(Self::Student),
			"company" => Ok(Self::Company),
			_ => Err(Status::InternalServerError),
		}
	}
}
