use lettre::{
	Message, SmtpTransport, Transport,
	message::{Mailbox, header::ContentType},
	transport::smtp::authentication::Credentials,
};
use rand::seq::{IndexedRandom, SliceRandom};
use regex::Regex;

#[must_use]
pub fn verify_mail(mail: &str) -> bool {
	let regex = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        ).unwrap();

	regex.is_match(mail)
}

pub fn send_2fa_mail() -> String {
	let mut code = vec![];

	let mut rng = rand::rng();
	for _ in 1..=6 {
		let mut nums: Vec<i32> = (0..=9).collect();
		nums.shuffle(&mut rng);
		code.push(nums.choose(&mut rng).unwrap().to_string());
	}

	let code = code.join("");

	let email = Message::builder()
		.from(Mailbox::new(
			Some("mosifratest".to_owned()),
			"mosifratest@gmail.com".parse().unwrap(),
		))
		.to(Mailbox::new(
			Some("nissya".to_owned()),
			"nissya@proton.me".parse().unwrap(),
		))
		.header(ContentType::TEXT_PLAIN)
		.body(code.clone())
		.unwrap();

	let creds = Credentials::new("mosifratest".to_owned(), "vftf jnbn peix uqvt".to_owned());

	let mailer = SmtpTransport::relay("smtp.gmail.com")
		.unwrap()
		.credentials(creds)
		.build();

	match mailer.send(&email) {
		Ok(_) => println!("Email sent successfully!"),
		Err(e) => panic!("Could not send email: {e:?}"),
	}

	code
}
