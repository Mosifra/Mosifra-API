use rocket::form::Form;

use crate::{
	db::insert_universite,
	types::university::{University, UniversityDto},
	utils::{send_2fa_mail, verify_mail},
};

#[post("/user/create_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_university(form: Form<UniversityDto>) -> Result<String, String> {
	let university =
		University::try_from(form.into_inner()).map_err(|()| "Conversion échouée".to_string())?;
	println!("{university:#?}");

	if verify_mail(&university.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	let code =
		send_2fa_mail(&university.mail).map_err(|()| "Échec de l’envoi du mail".to_string())?;
	println!("code needs to be {code}");

	insert_universite(
		university.name,
		university.mail,
		university.login,
		code,
		university.password,
	)
	.await
}
