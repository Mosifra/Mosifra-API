use std::process::exit;

use rocket::{
	Config,
	figment::{Figment, providers::Env},
	http::Method,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
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
	user::{
		delete::{company::delete_company, university::delete_university},
		get::{
			companies::get_companies,
			student::{course_type::get_student_course_type, info::get_student_info},
			universities::get_universities,
			university::course_types::get_university_course_types,
			user_type::get_user_type,
		},
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

#[derive(OpenApi)]
#[openapi(
	paths(
		routes::auth::login::login,
		routes::auth::logout::logout,
		routes::auth::session::check_session,
		routes::auth::twofa::twofa,
		routes::courses::delete::class::delete_class,
		routes::courses::get::classes::get_classes,
		routes::courses::get::internships::get_internships,
		routes::courses::get::class::students::get_class_students,
		routes::create::class::create_class,
		routes::create::company::create_company,
		routes::create::internship::create_internship,
		routes::create::students::create_students,
		routes::create::university::create_university,
		routes::user::delete::company::delete_company,
		routes::user::delete::university::delete_university,
		routes::user::get::companies::get_companies,
		routes::user::get::universities::get_universities,
		routes::user::get::user_type::get_user_type,
		routes::user::get::student::course_type::get_student_course_type,
		routes::user::get::student::info::get_student_info,
		routes::user::get::university::course_types::get_university_course_types,
	),
	components(
		schemas(
			// Auth
			routes::auth::LoginPayload,
			routes::auth::LoginResponse,
			routes::auth::TwofaPayload,
			routes::auth::TwofaResponse,
			routes::auth::CheckSessionResponse,
			routes::auth::DisconnectResponse,
			// Courses
			routes::courses::delete::domain::DeleteClassPayload,
			routes::courses::delete::domain::DeleteClassResponse,
			routes::courses::get::domain::GetClassesResponse,
			routes::courses::get::domain::GetInternshipsPayload,
			routes::courses::get::domain::GetInternshipsResponse,
			routes::courses::get::class::domain::GetClassStudentsPayload,
			routes::courses::get::class::domain::GetClassStudentsResponse,
			// Create
			routes::create::domain::CreateCompanyPayload,
			routes::create::domain::CreateUserResponse,
			routes::create::domain::StudentCsvPayload,
			routes::create::domain::StudentCsvResponse,
			routes::create::domain::CreateUniversityPayload,
			routes::create::domain::CreateClassPayload,
			routes::create::domain::CreateClassResponse,
			routes::create::domain::CreateIntershipPayload,
			routes::create::domain::CreateInternshipResponse,
			// User
			routes::user::delete::domain::DeleteCompanyResponse,
			routes::user::delete::domain::DeleteCompanyPayload,
			routes::user::delete::domain::DeleteUniversityResponse,
			routes::user::delete::domain::DeleteUniversityPayload,
			routes::user::get::domain::GetUserTypeResponse,
			routes::user::get::domain::GetUniversitiesResponse,
			routes::user::get::domain::GetCompaniesResponse,
			routes::user::get::student::domain::GetInfoResponse,
			routes::user::get::student::domain::GetCourseTypeResponse,
			routes::user::get::university::domain::GetCourseTypesResponse,
			// Models
			models::courses::CourseType,
			models::courses::dto::class::ClassDto,
			models::courses::Internship,
			models::users::dto::StudentDto,
			models::users::Company,
			models::users::University,
			models::auth::UserType,
		)
	),
	tags(
		(name = "Auth", description = "Authentication routes"),
		(name = "Courses", description = "Courses routes"),
		(name = "Create", description = "Creation routes"),
		(name = "User", description = "User routes"),
	)
)]
struct ApiDoc;

#[derive(Debug, PartialEq, Deserialize)]
struct Environment {
	rocket_secret: String,
	api_port: usize,
}

#[launch]
fn rocket() -> _ {
	let env: Environment = Figment::from(Env::raw().only(&["rocket_secret", "api_port"]))
		.extract()
		.unwrap_or_else(|e| {
			eprintln!("Error while trying to get the env: {e}");
			exit(1);
		});

	let rocket = rocket::custom(Config::from(
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
				delete_company,
				delete_university,
			],
		)
		.mount("/", SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()))
		.attach(cors.to_cors().unwrap())
}
