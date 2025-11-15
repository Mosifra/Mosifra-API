use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::http::Status;

use crate::error_handling::StatusResultHandling;

#[allow(clippy::missing_errors_doc)]
pub fn verify_password(pwd_to_check: &str, stored_hash: &str) -> Result<bool, Status> {
	let parsed_hash =
		PasswordHash::new(stored_hash).internal_server_error("Erreur parsing hash")?;
	let is_correct = Argon2::default()
		.verify_password(pwd_to_check.as_bytes(), &parsed_hash)
		.map(|()| true);

	Ok(is_correct.is_ok())
}
