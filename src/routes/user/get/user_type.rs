use rocket::{http::Status, serde::json::Json};

use crate::structs::jwt::UserJwt;

use super::domain::{GetUserTypePayload, GetUserTypeResponse};

#[post("/user/user_type", data = "<get_user_type_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_user_type(
	get_user_type_payload: Json<GetUserTypePayload>,
) -> Result<Json<GetUserTypeResponse>, Status> {
	let raw_jwt = get_user_type_payload.into_inner().jwt;

	let user_jwt = UserJwt::from_raw_jwt(&raw_jwt)?;
	match user_jwt {
		Some(user_jwt) => Ok(Json(GetUserTypeResponse {
			user_type: Some(user_jwt.user_type),
		})),
		None => Ok(Json(GetUserTypeResponse { user_type: None })),
	}
}
