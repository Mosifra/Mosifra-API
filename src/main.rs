use std::{env, process::exit};

use rocket::{Config, http::Method};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{
	auth::{check_session, login_route, twofa_route},
	courses::get::classes::get_classes,
	create::{
		class::create_class, company::create_company, students::create_students,
		university::create_university,
	},
	user::get::{info::get_student_info, user_type::get_user_type},
};

mod error_handling;
pub mod models;
pub mod postgres;
pub mod redis;
pub mod routes;
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
				login_route,
				twofa_route,
				check_session,
				create_company,
				create_students,
				create_university,
				get_user_type,
				create_class,
				get_student_info,
				get_classes,
			],
		)
		.attach(cors.to_cors().unwrap())
}
