use std::env;

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

	let rocket_secret = env::var("ROCKET_SECRET")
		.ok()
		.expect("SECRET is to be specified in .env");

	let rocket = rocket::custom(Config::from(
		Config::figment().merge(("secret_key", rocket_secret)),
	));

	rocket.mount("/", routes![user::create_university])
}
