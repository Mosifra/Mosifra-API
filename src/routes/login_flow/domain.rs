#[derive(Debug, FromForm)]
pub struct Login {
	pub login: String,
	pub password: String,
	pub remember_me: bool,
	pub user_type: String,
}

#[derive(Debug, FromForm)]
pub struct Twofa {
	pub code: String,
	pub transaction_id: String,
	pub user_type: String,
	pub remember_me: bool,
}
