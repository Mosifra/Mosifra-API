use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rocket::http::Status;
 
#[allow(clippy::missing_errors_doc)]
pub fn generate_password() -> Result<String, Status> {
	const CHARSET_UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
	const CHARSET_LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
	const CHARSET_NUMBER: &[u8] = b"0123456789";
	const CHARSET_SYMBOL: &[u8] = b"!/?";

	let mut rng = rand::rng();
	let mut password_chars = Vec::with_capacity(8);

	password_chars.push(*CHARSET_UPPER.choose(&mut rng).unwrap() as char);
	password_chars.push(*CHARSET_LOWER.choose(&mut rng).unwrap() as char);
	password_chars.push(*CHARSET_NUMBER.choose(&mut rng).unwrap() as char);
	password_chars.push(*CHARSET_SYMBOL.choose(&mut rng).unwrap() as char);

	let all_chars = [CHARSET_UPPER, CHARSET_LOWER, CHARSET_NUMBER, CHARSET_SYMBOL].concat();
	for _ in 0..4 {
		password_chars.push(*all_chars.choose(&mut rng).unwrap() as char);
	}

	password_chars.shuffle(&mut rng);

	Ok(password_chars.into_iter().collect())
}
