use std::{env, process::exit};

use rocket::Config;
use routes::user;

pub mod routes;
pub mod types;
pub mod utils;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
	dotenvy::dotenv().expect("Error while loading .env");

	let rocket_secret = env::var("ROCKET_SECRET").ok().map_or_else(
		|| {
			eprintln!("Secret must be in .env");
			exit(1)
		},
		|secret| secret,
	);

	let rocket = rocket::custom(Config::from(
		Config::figment().merge(("secret_key", rocket_secret)),
	));

	rocket.mount("/", routes![user::create_university])
}
