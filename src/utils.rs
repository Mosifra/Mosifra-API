use argon2::{
	Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
	password_hash::{SaltString, rand_core::OsRng},
};

use lettre::{
	Message, SmtpTransport, Transport,
	message::{Mailbox, header::ContentType},
	transport::smtp::authentication::Credentials,
};
use passwords::PasswordGenerator;
use rand::seq::{IndexedRandom, SliceRandom};
use regex::Regex;

#[must_use]
#[allow(
	clippy::missing_panics_doc,
	clippy::result_unit_err,
	clippy::missing_errors_doc
)] // WIP
pub fn verify_mail(mail: &str) -> bool {
	// Should never crash
	#[allow(clippy::unwrap_used)]
	let regex = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        ).unwrap();

	regex.is_match(mail)
}

#[allow(
	clippy::missing_panics_doc,
	clippy::result_unit_err,
	clippy::missing_errors_doc
)] // WIP
pub async fn send_2fa_mail(to: &str) -> Result<String, String> {
	let mut code = vec![];

	let mut rng = rand::rng();
	for _ in 1..=6 {
		let mut nums: Vec<i32> = (0..=9).collect();
		nums.shuffle(&mut rng);
		#[allow(clippy::unwrap_used)] // WIP
		code.push(nums.choose(&mut rng).unwrap().to_string());
	}

	let code = code.join("");

	#[allow(clippy::unwrap_used)] //WIP
	let email = Message::builder()
		.from(Mailbox::new(None, "mosifratest@gmail.com".parse().unwrap()))
		.to(Mailbox::new(None, to.parse().unwrap()))
		.header(ContentType::TEXT_PLAIN)
		.body(code.clone())
		.unwrap();

	let creds = Credentials::new("mosifratest".to_owned(), "vftf jnbn peix uqvt".to_owned());

	#[allow(clippy::unwrap_used)] //WIP
	let mailer = SmtpTransport::relay("smtp.gmail.com")
		.unwrap()
		.credentials(creds)
		.build();

	match mailer.send(&email) {
		Ok(_) => println!("Email sent successfully!"),
		Err(e) => panic!("Could not send email: {e:?}"),
	}

	Ok(code)
}

#[allow(clippy::missing_errors_doc)]
pub fn verify_password(pwd_to_check: &str, stored_hash: &str) -> Result<bool, String> {
	let parsed_hash =
		PasswordHash::new(stored_hash).map_err(|e| format!("Erreur parsing hash: {e}"))?;
	Argon2::default()
		.verify_password(pwd_to_check.as_bytes(), &parsed_hash)
		.map(|()| true)
		.map_err(|_| "Mot de passe incorrect".to_string())
}

#[allow(clippy::missing_errors_doc)]
pub fn hash_password(password: &str) -> Result<String, String> {
	let bytes_password = password.as_bytes();
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();

	let password_hash = argon2
		.hash_password(bytes_password, &salt)
		.map_err(|e| format!("Error while tryinng to hash password {e}"))?
		.to_string();

	Ok(password_hash)
}

#[allow(clippy::missing_errors_doc)]
pub fn generate_password() -> Result<String, &'static str> {
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
}

// #[allow(clippy::missing_errors_doc)]
// pub fn create_jwt(user_type: String, user_id: String) {
	
// }
