use std::{collections::BTreeMap, env, process::exit};

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::{redis::session_exist, structs::user_type::UserType};

#[derive(Debug)]
pub struct UserJwt {
	pub session_id: String,
	pub user_type: UserType,
}

impl UserJwt {
	pub fn from_raw_jwt(raw_jwt: &str) -> Result<Self, String> {
		let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
			|| {
				eprintln!("JWT Secret must be in .env");
				exit(1)
			},
			|secret| secret,
		);
		let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
			.map_err(|e| format!("Error getting key from JWT secret : {e}"))?;

		let claims: BTreeMap<String, String> = raw_jwt
			.verify_with_key(&key)
			.map_err(|e| format!("Error getting claims on jwt token : {e}"))?;

		let user_type = match claims["user_type"].as_str() {
			"admin" => UserType::Admin,
			"student" => UserType::Student,
			"company" => UserType::Company,
			"university" => UserType::University,
			_ => return Err("Incorrect UserType".to_string()),
		};

		let session_id = claims["session_id"].clone();

		if session_exist(&session_id)? {
			Ok(Self {
				session_id,
				user_type,
			})
		} else {
			Err("Invalid session".to_string())
		}
	}

	pub fn new_raw_jwt_from_data(
		session_id: String,
		user_type: &UserType,
	) -> Result<String, String> {
		if !session_exist(&session_id)? {
			return Err("Invalid session".to_string());
		}

		let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
			|| {
				eprintln!("JWT Secret must be in .env");
				exit(1)
			},
			|secret| secret,
		);

		let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
			.map_err(|e| format!("Error getting key from JWT secret : {e}"))?;

		let mut claims = BTreeMap::new();
		claims.insert("session_id", session_id);
		claims.insert("user_type", user_type.to_string());

		let token = claims
			.sign_with_key(&key)
			.map_err(|e| format!("Error creating jwt token : {e}"))?;

		Ok(token)
	}

	pub fn can_access_admin_pages(&self) -> bool {
		self.user_type == UserType::Admin
	}

	pub fn can_access_university_pages(&self) -> bool {
		self.user_type == UserType::University
	}

	pub fn can_access_student_pages(&self) -> bool {
		self.user_type == UserType::Student
	}

	pub fn can_access_company_pages(&self) -> bool {
		self.user_type == UserType::Company
	}
}
