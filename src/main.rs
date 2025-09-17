use routes::user;

pub mod routes;
pub mod types;
pub mod utils;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![user::create_university])
}
