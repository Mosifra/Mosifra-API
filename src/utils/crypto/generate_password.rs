use passwords::PasswordGenerator;
use rocket::http::Status;

use crate::error_handling::StatusResultHandling;

#[allow(clippy::missing_errors_doc)]
pub fn generate_password() -> Result<String, Status> {
	PasswordGenerator::new()
		.length(8)
		.numbers(true)
		.lowercase_letters(true)
		.uppercase_letters(true)
		.symbols(true)
		.spaces(false)
		.exclude_similar_characters(true)
		.strict(true)
		.generate_one()
		.internal_server_error("Error while generating password")
}
