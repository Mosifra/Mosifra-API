use std::{collections::BTreeMap, env, process::exit};

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::{
	Request,
	http::Status,
	request::{FromRequest, Outcome},
};
use sha2::Sha256;

use crate::{
	error_handling::StatusResultHandling,
	models::users::{GenericUser, Student, University},
	redis::{self, session_exist},
};

use super::UserType;

#[derive(Debug)]
pub struct AuthGuard {
	pub session_id: String,
	pub user_type: UserType,
}

impl AuthGuard {
	fn from_raw_jwt(raw_jwt: &str) -> Result<Self, String> {
		let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
			|| {
				eprintln!("JWT Secret must be in .env");
				exit(1)
			},
			|secret| secret,
		);
		let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
			.map_err(|e| format!("Error getting key from JWT secret: {e}"))?;

		let claims: BTreeMap<String, String> = raw_jwt
			.verify_with_key(&key)
			.map_err(|e| format!("Error getting claims on jwt token : {e}"))?;

		let user_type = match claims["user_type"].as_str() {
			"admin" => UserType::Admin,
			"student" => UserType::Student,
			"company" => UserType::Company,
			"university" => UserType::University,
			_ => return Err("Incorrect user_type".to_string()),
		};

		let session_id = claims["session_id"].clone();

		Ok(Self {
			session_id,
			user_type,
		})
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

		let jwt = claims
			.sign_with_key(&key)
			.internal_server_error("Error creating jwt token")?;

		Ok(Some(jwt))
	}

	pub async fn get_generic_user(&self) -> Result<GenericUser, Status> {
		match self.user_type {
			UserType::Admin => todo!(),
			UserType::University => Ok(GenericUser::new(
				University::from_id(self.get_user_id()?),
				self.session_id.clone(),
			)),
			UserType::Student => Ok(GenericUser::new(
				Student::from_id(self.get_user_id()?),
				self.session_id.clone(),
			)),
			UserType::Company => todo!(),
		}
	}

	pub fn get_user_id(&self) -> Result<String, Status> {
		redis::get_user_id_from_session_id(self.session_id.clone())
	}
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
	type Error = String;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let auth_header = request.headers().get_one("Authorization");
		match auth_header {
			Some(header) if header.starts_with("Bearer ") => {
				let jwt = header.trim_start_matches("Bearer ");
				let is_correct = match validate_jwt(jwt) {
					Ok(is_correct) => is_correct,
					Err(e) => {
						return Outcome::Error((e, e.to_string()));
					}
				};
				if is_correct {
					let auth_guard = match Self::from_raw_jwt(jwt) {
						Ok(auth_guard) => auth_guard,
						Err(e) => {
							return Outcome::Error((
								Status::InternalServerError,
								format!(
									"Error while getting the jwt information (Should be impossible ?) : {e}"
								),
							));
						}
					};
					let session_exist = match session_exist(&auth_guard.session_id) {
						Ok(session_exist) => session_exist,
						Err(e) => {
							return Outcome::Error((e, "Error while checking session".to_string()));
						}
					};
					if session_exist {
						Outcome::Success(auth_guard)
					} else {
						rocket::outcome::Outcome::Error((
							Status::Unauthorized,
							"Session expired".to_string(),
						))
					}
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

fn validate_jwt(jwt: &str) -> Result<bool, Status> {
	let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
		|| {
			eprintln!("JWT Secret must be in .env");
			exit(1)
		},
		|secret| secret,
	);
	let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
		.internal_server_error("Error getting key from JWT secret")?;

	let claims: Result<BTreeMap<String, String>, jwt::Error> = jwt.verify_with_key(&key);

	Ok(claims.is_ok())
}
