use rocket::{http::Status, serde::json::Json};

use crate::{
	models::{auth::AuthGuard, users::University},
	postgres::Db,
	utils::mail::verify_mail,
};

use super::domain::{CreateUniversityPayload, CreateUserResponse};

#[post("/create/university", data = "<create_university_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_university(
	auth: AuthGuard,
	create_university_payload: Json<CreateUniversityPayload>,
) -> Result<Json<CreateUserResponse>, Status> {
	let generic_user = auth.get_generic_user().await?;

	if generic_user.is_admin() {
		let university = University::try_from(create_university_payload.into_inner())?;

		if !verify_mail(&university.mail)? {
			return Ok(Json(CreateUserResponse {
				success: false,
				password: None,
			}));
		}

		let is_inserted = university.insert().await;

		if is_inserted.is_ok() {
			Ok(Json(CreateUserResponse {
				success: true,
				password: Some(university.password),
			}))
		} else {
			Ok(Json(CreateUserResponse {
				success: false,
				password: None,
			}))
		}
	} else {
		Err(Status::Unauthorized)
	}
}
