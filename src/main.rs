use rocket::form::Form;
use university::{University, UniversityDto};
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
fn create_university(form: Form<UniversityDto>) -> Result<(), ()> {
	let university = University::try_from(form.into_inner())?;
	println!("{university:#?}");
	if verify_mail(&university.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	println!("code needs to be {}", send_2fa_mail()?);
	Ok(())
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![create_university])
}
