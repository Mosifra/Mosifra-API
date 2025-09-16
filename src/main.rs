use lettre::{
	Message, SmtpTransport, Transport,
	message::{Mailbox, header::ContentType},
	transport::smtp::authentication::Credentials,
};

use rocket::form::Form;
use university::UniversityDto;
use utils::{send_2fa_mail, verify_mail};

pub mod class;
pub mod course_type;
pub mod internship;
pub mod role;
pub mod university;
pub mod utils;

#[macro_use]
extern crate rocket;

#[post("/user/create_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
fn create_university(form: Form<UniversityDto>) {
	println!("{form:#?}");
	if verify_mail(&form.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	println!("code needs to be {}", send_2fa_mail());
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![create_university])
}
