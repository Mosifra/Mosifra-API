use argon2::{
	Argon2, PasswordHasher,
	password_hash::{SaltString, rand_core::OsRng},
};
use rocket::http::Status;

use crate::error_handling::StatusResultHandling;

#[allow(clippy::missing_errors_doc)]
pub fn hash_password(password: &str) -> Result<String, Status> {
	let bytes_password = password.as_bytes();
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();

	let password_hash = argon2
		.hash_password(bytes_password, &salt)
		.internal_server_error("Error while tryinng to hash password")?
		.to_string();

	Ok(password_hash)
}
