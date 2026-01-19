use std::process::exit;

use rocket::{
	Config,
	figment::{Figment, providers::Env},
	http::Method,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{
	auth::{check_session, login_route, logout_route, twofa_route},
	courses::{
		delete::class::delete_class,
		get::{
			class::students::get_class_students, classes::get_classes, internships::get_internships,
		},
	},
	create::{
		class::create_class, company::create_company, internship::create_internship,
		students::create_students, university::create_university,
	},
	user::get::{
		companies::get_companies,
		student::{course_type::get_student_course_type, info::get_student_info},
		universities::get_universities,
		university::course_types::get_university_course_types,
		user_type::get_user_type,
	},
};
use serde::Deserialize;

mod error_handling;
pub mod models;
pub mod postgres;
pub mod redis;
pub mod routes;
pub mod utils;

#[macro_use]
extern crate rocket;

#[derive(Debug, PartialEq, Deserialize)]
struct Environment {
	rocket_secret: String,
	api_port: usize,
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
	let env: Environment = Figment::from(Env::raw().only(&["rocket_secret", "api_port"]))
		.extract()
		.unwrap_or_else(|e| {
			eprintln!("Error while trying to get the env: {e}");
			exit(1);
		});

	let rocket = rocket::custom(Config::from(
		Config::figment().merge(("secret_key", rocket_secret)),
		Config::figment()
			.merge(("secret_key", env.rocket_secret))
			.merge(("port", env.api_port)),
	));

	let cors = CorsOptions::default()
		.allowed_origins(AllowedOrigins::all())
		.allowed_methods(
			vec![
				Method::Get,
				Method::Post,
				Method::Patch,
				Method::Options,
				Method::Delete,
			]
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
				get_class_students,
				delete_class,
				logout_route,
				create_internship,
				get_internships,
				get_university_course_types,
				get_student_course_type,
				get_companies,
				get_universities,
			],
		)
		.attach(cors.to_cors().unwrap())
}
