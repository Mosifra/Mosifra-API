use rocket::form::Form;

use crate::{
	types::university::{University, UniversityDto},
	utils::{send_2fa_mail, verify_mail},
};

#[post("/user/create_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
pub fn create_university(form: Form<UniversityDto>) -> Result<(), ()> {
	let university = University::try_from(form.into_inner())?;
	println!("{university:#?}");
	if verify_mail(&university.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	println!("code needs to be {}", send_2fa_mail(university.mail)?);
	Ok(())
}
