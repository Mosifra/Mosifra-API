use std::{env, process::exit};

use rocket::Config;
use routes::{login, user};

pub mod routes;
pub mod structs;
pub mod traits;
pub mod utils;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
	match dotenvy::dotenv() {
		Ok(_) => (),
		Err(e) => {
			eprintln!("Error while loading .env : {e}");
			exit(1)
		}
	}

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

	rocket.mount(
		"/",
		routes![
			user::create_university,
			user::create_student,
			user::create_company,
			login::login_university,
			login::login_student,
			login::login_company
		],
	)
}
