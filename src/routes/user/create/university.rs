use rocket::{http::Status, serde::json::Json};

use crate::{structs::university::University, traits::db::Db, utils::verify_mail};

use super::domain::{CreateUniversityPayload, CreateUniversityResponse};

#[post("/user/create_university", data = "<create_university_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_university(
	create_university_payload: Json<CreateUniversityPayload>,
) -> Result<Json<CreateUniversityResponse>, Status> {
	let university = University::try_from(create_university_payload.into_inner())?;

	println!("==========DEBUG==========");
	println!("login : {}", university.login);
	println!("password : {}", university.password);
	println!("==========DEBUG==========");

	if verify_mail(&university.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	let is_inserted = university.insert().await;

	Ok(Json(CreateUniversityResponse {
		success: is_inserted.is_ok(),
	}))
}
