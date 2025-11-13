use std::path::PathBuf;

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
use rocket::{http::Status, serde::json::Json};

use crate::{
	redis::get_transactionid,
	routes::login_flow::domain::LoginResponse,
	structs::student::Student,
	traits::{
		db::{Db, is_login_taken},
		status::{StatusOptionHandling, StatusResultHandling},
	},
};

#[must_use]
#[allow(
	clippy::missing_panics_doc,
	clippy::result_unit_err,
	clippy::missing_errors_doc
)] // WIP
pub fn verify_mail(mail: &str) -> Result<bool, Status> {
	let regex = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        ).internal_server_error("Failed to build email string")?;

	Ok(regex.is_match(mail))
}

#[allow(
	clippy::missing_panics_doc,
	clippy::result_unit_err,
	clippy::missing_errors_doc
)] // WIP
pub fn send_2fa_mail(to: &str) -> Result<String, Status> {
	let mut code = vec![];

	let mut rng = rand::rng();
	for _ in 1..=6 {
		let mut nums: Vec<i32> = (0..=9).collect();
		nums.shuffle(&mut rng);
		code.push(
			nums.choose(&mut rng)
				.internal_server_error("Failed to choose number rng")?
				.to_string(),
		);
	}

	let code = code.join("");

	let email = Message::builder()
		.from(Mailbox::new(
			None,
			"mosifratest@gmail.com"
				.parse()
				.internal_server_error("Error while parsing 'from' email")?,
		))
		.to(Mailbox::new(
			None,
			to.parse()
				.internal_server_error("Error while parsing 'to' email")?,
		))
		.header(ContentType::TEXT_PLAIN)
		.body(code.clone())
		.internal_server_error("Error while building email")?;

	let creds = Credentials::new("mosifratest".to_owned(), "vftf jnbn peix uqvt".to_owned()); // need to go in .env

	let mailer = SmtpTransport::relay("smtp.gmail.com")
		.internal_server_error_no_message()?
		.credentials(creds)
		.build();

	mailer
		.send(&email)
		.internal_server_error("Error email failed to send")?;

	Ok(code)
}

#[allow(clippy::missing_errors_doc)]
pub fn verify_password(pwd_to_check: &str, stored_hash: &str) -> Result<bool, Status> {
	let parsed_hash =
		PasswordHash::new(stored_hash).internal_server_error("Erreur parsing hash")?;
	let is_correct = Argon2::default()
		.verify_password(pwd_to_check.as_bytes(), &parsed_hash)
		.map(|()| true);

	Ok(is_correct.is_ok())
}

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

// Yaniss Lasbordes -> ylasbordes1 if already exist ylasbordes2 until ylasbordesn

pub async fn generate_login(first_name: &str, last_name: &str) -> Result<String, Status> {
	let first_name = first_name.to_lowercase();
	let last_name = last_name.to_lowercase();
	let first_name_letter = first_name
		.chars()
		.next()
		.internal_server_error("Login generation error : login is empty")?;
	let mut res;
	let mut i = 1;

	loop {
		res = format!("{first_name_letter}{last_name}{i}");
		if !is_login_taken(&res).await? {
			break;
		}
		i += 1;
	}

	Ok(res)
}

pub async fn read_csv(file_path: PathBuf) -> Result<(), Status> {
	let mut reader =
		csv::Reader::from_path(file_path).internal_server_error("Failed to creatre reader")?;

	for result in reader.records() {
		let record = result.internal_server_error("Failed to read string record")?;

		let student = Student::from_record(record).await?;
		student.insert().await?;
	}

	Ok(())
}

pub async fn set_transaction_id(
	mail: &str,
	id: &str,
	remember_me: bool,
) -> Result<Json<LoginResponse>, Status> {
	let code = send_2fa_mail(mail)?;
	let transaction_id = get_transactionid(id, code)?;
	Ok(Json(LoginResponse {
		valid: true,
		transaction_id: Some(transaction_id),
		remember_me: Some(remember_me),
	}))
}
