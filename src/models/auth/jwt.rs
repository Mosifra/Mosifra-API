use std::{collections::BTreeMap, env, process::exit};

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::{
	Request,
	http::Status,
	request::{FromRequest, Outcome},
	serde::json::Json,
};
use sha2::Sha256;

use crate::{
	error_handling::{StatusOptionHandling, StatusResultHandling},
	models::{courses::get::domain::GetClassesResponse, users::Student},
	redis::{self, session_exist},
	routes::user::get::domain::GetInfoResponse,
};

use super::UserType;

#[derive(Debug)]
pub struct AuthGuard;

#[async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
	type Error = String;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let auth_header = request.headers().get_one("Authorization");
		match auth_header {
			Some(header) if header.starts_with("Bearer ") => {
				let token = header.trim_start_matches("Bearer ");
				let is_correct = match validate_jwt(token.to_string()) {
					Ok(is_correct) => is_correct,
					Err(e) => {
						return Outcome::Error((e, e.to_string()));
					}
				};
				if is_correct {
					Outcome::Success(AuthGuard)
				} else {
					Outcome::Error((Status::Unauthorized, "Invalid Token".to_string()))
				}
			}
			_ => Outcome::Error((
				Status::Unauthorized,
				"Authorization header missing".to_string(),
			)),
		}
	}
}

fn validate_jwt(jwt: String) -> Result<bool, Status> {
	let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
		|| {
			eprintln!("JWT Secret must be in .env");
			exit(1)
		},
		|secret| secret,
	);
	let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
		.internal_server_error("Error getting key from JWT secret")?;

	Ok(jwt.verify_with_key(&key).is_ok())
}

#[derive(Debug)]
pub struct UserJwt {
	pub session_id: String,
	pub user_type: UserType,
}

impl UserJwt {
	pub fn from_raw_jwt(raw_jwt: &str) -> Result<Option<Self>, Status> {
		let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
			|| {
				eprintln!("JWT Secret must be in .env");
				exit(1)
			},
			|secret| secret,
		);
		let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
			.internal_server_error("Error getting key from JWT secret")?;

		let claims: BTreeMap<String, String> = raw_jwt
			.verify_with_key(&key)
			.internal_server_error("Error getting claims on jwt token")?;

		let user_type = match claims["user_type"].as_str() {
			"admin" => UserType::Admin,
			"student" => UserType::Student,
			"company" => UserType::Company,
			"university" => UserType::University,
			_ => return Ok(None),
		};

		let session_id = claims["session_id"].clone();

		if session_exist(&session_id)? {
			Ok(Some(Self {
				session_id,
				user_type,
			}))
		} else {
			Ok(None)
		}
	}

	pub fn new_raw_jwt_from_data(
		session_id: String,
		user_type: &UserType,
	) -> Result<Option<String>, Status> {
		if !session_exist(&session_id)? {
			return Ok(None);
		}

		let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
			|| {
				eprintln!("JWT Secret must be in .env");
				exit(1)
			},
			|secret| secret,
		);

		let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
			.internal_server_error("Error getting key from JWT secret")?;

		let mut claims = BTreeMap::new();
		claims.insert("session_id", session_id);
		claims.insert("user_type", user_type.to_string());

		let token = claims
			.sign_with_key(&key)
			.internal_server_error("Error creating jwt token")?;

		Ok(Some(token))
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

	pub async fn get_student_info(&self) -> Result<Json<GetInfoResponse>, Status> {
		let user_id = redis::get_user_id_from_session_id(self.session_id.clone())?;
		let student = Student::from_user_id(user_id).await?;
		let class = student
			.get_class()
			.await?
			.internal_server_error("This student has no class")?;
		let university = class.get_university().await?;

		Ok(Json(GetInfoResponse {
			success: true,
			first_name: Some(student.first_name),
			last_name: Some(student.last_name),
			email: Some(student.mail),
			university: Some(university.name),
			class_name: Some(class.name),
		}))
	}

	pub async fn get_classes(&self) -> Result<Json<GetClassesResponse>, Status> {
		if self.user_type != UserType::University {
			return Ok(Json(GetClassesResponse {
            success: false,
            classes: None,
		}))
		} else {
			todo!("À finir")
		}
	}
}
