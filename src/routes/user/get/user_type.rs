use rocket::form::Form;
use serde_json::json;

use crate::{routes::session::domain::Jwt, structs::jwt::UserJwt};

#[get("/user/user_type", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_user_type(form: Form<Jwt>) -> Result<String, String> {
	let raw_jwt = form.into_inner().jwt;

	let user_type = UserJwt::from_raw_jwt(&raw_jwt)?.user_type.to_string();
	Ok(json!({"user_type": user_type}).to_string())
}
