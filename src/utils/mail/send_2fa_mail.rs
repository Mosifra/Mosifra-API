use lettre::{
	Message, SmtpTransport, Transport,
	message::{Mailbox, header::ContentType},
	transport::smtp::authentication::Credentials,
};
use rand::seq::{IndexedRandom, SliceRandom};
use rocket::http::Status;

use crate::error_handling::{StatusOptionHandling, StatusResultHandling};

#[allow(clippy::missing_errors_doc)]
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
