use rocket::form::Form;

use crate::{
	structs::university::{University, UniversityDto},
	traits::db::Db,
	utils::verify_mail,
};

#[post("/user/create_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_university(form: Form<UniversityDto>) -> Result<String, String> {
	let university = University::try_from(form.into_inner())
		.map_err(|()| "Error while converting UniversityDto".to_string())?; // Not ideal but will do
	println!("{university:#?}");

	if verify_mail(&university.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	university.insert().await
}
