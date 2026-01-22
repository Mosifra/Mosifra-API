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
	let mut code = String::new();

	for _ in 0..6 {
    	let num = rng.random_range(0..10);
    	code.push_str(&num.to_string());
	}

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
