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
	traits::db::{Db, is_login_taken},
};

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
pub async fn send_2fa_mail(to: &str) -> Result<String, Status> {
	let mut code = vec![];

	let mut rng = rand::rng();
	for _ in 1..=6 {
		let mut nums: Vec<i32> = (0..=9).collect();
		nums.shuffle(&mut rng);
		#[allow(clippy::unwrap_used)] // Should never crash
		code.push(nums.choose(&mut rng).unwrap().to_string());
	}

	let code = code.join("");

	let email = Message::builder()
		.from(Mailbox::new(
			None,
			"mosifratest@gmail.com".parse().map_err(|e| {
				eprintln!("Error while parsing 'from' email : {e}");
				Status::InternalServerError
			})?,
		))
		.to(Mailbox::new(
			None,
			to.parse().map_err(|e| {
				eprintln!("Error while parsing 'to' email : {e}");
				Status::InternalServerError
			})?,
		))
		.header(ContentType::TEXT_PLAIN)
		.body(code.clone())
		.map_err(|e| {
			eprintln!("Error while building email : {e}");
			Status::InternalServerError
		})?;

	let creds = Credentials::new("mosifratest".to_owned(), "vftf jnbn peix uqvt".to_owned()); // need to go in .env

	let mailer = SmtpTransport::relay("smtp.gmail.com")
		.map_err(|e| {
			eprintln!("{e}");
			Status::InternalServerError
		})?
		.credentials(creds)
		.build();

	match mailer.send(&email) {
		Ok(_) => println!("Email sent successfully!"),
		Err(e) => {
			eprintln!("Error email failed to send : {e}");
			return Err(Status::InternalServerError);
		}
	}

	Ok(code)
}

#[allow(clippy::missing_errors_doc)]
pub fn verify_password(pwd_to_check: &str, stored_hash: &str) -> Result<bool, Status> {
	let parsed_hash = PasswordHash::new(stored_hash).map_err(|e| {
		eprintln!("Erreur parsing hash: {e}");
		Status::InternalServerError
	})?;
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
		.map_err(|e| {
			eprintln!("Error while tryinng to hash password {e}");
			Status::InternalServerError
		})?
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
		.map_err(|e| {
			eprintln!("Error while generating password : {e}");
			Status::InternalServerError
		})
}

// Yaniss Lasbordes -> ylasbordes1 if already exist ylasbordes2 until ylasbordesn

pub async fn generate_login(first_name: &str, last_name: &str) -> Result<String, Status> {
	let first_name = first_name.to_lowercase();
	let last_name = last_name.to_lowercase();
	let Some(first_name_letter) = first_name.chars().next() else {
		eprintln!("Login generation error : login is empty");
		return Err(Status::InternalServerError);
	};
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
	let mut reader = csv::Reader::from_path(file_path).unwrap();

	for result in reader.records() {
		let record = result.unwrap();

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
	let code = send_2fa_mail(mail).await?;
	let transaction_id = get_transactionid(id, code)?;
	Ok(Json(LoginResponse {
		valid: true,
		transaction_id: Some(transaction_id),
		remember_me: Some(remember_me),
	}))
}
