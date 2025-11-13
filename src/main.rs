use std::{env, process::exit};

use rocket::{Config, http::Method};
use rocket_cors::{AllowedOrigins, CorsOptions};

pub mod redis;
pub mod routes;
pub mod structs;
pub mod traits;
pub mod utils;

#[macro_use]
extern crate rocket;

use routes::login_flow::login::login;
use routes::login_flow::twofa::twofa;
use routes::session::session::check_session;
use routes::user::create::company::create_company;
use routes::user::create::student::student_csv;
use routes::user::create::university::create_university;
use routes::user::get::user_type::get_user_type;

#[options("/<path..>")]
fn cors_preflight(path: std::path::PathBuf) -> &'static str {
    ""
}

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
			eprintln!("Rocket secret must be in .env");
			exit(1)
		},
		|secret| secret,
	);

	let rocket = rocket::custom(Config::from(
		Config::figment().merge(("secret_key", rocket_secret)),
	));

	let cors = CorsOptions::default()
		.allowed_origins(AllowedOrigins::all())
		.allowed_methods(
			vec![Method::Get, Method::Post, Method::Patch, Method::Options]
				.into_iter()
				.map(From::from)
				.collect(),
		)
		.allow_credentials(true);

	rocket
		.mount(
			"/",
			routes![
				login,
				twofa,
				check_session,
				create_company,
				student_csv,
				create_university,
				get_user_type,
				cors_preflight
			],
		)
		.attach(cors.to_cors().unwrap())
}
