use rocket::form::Form;
use serde_json::json;

use crate::structs::jwt::UserJwt;

use super::domain::Jwt;

use crate::redis;

#[get("/check_session", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn check_session(form: Form<Jwt>) -> Result<String, String> {
	let jwt = &form.into_inner().jwt;
	let user_jwt = UserJwt::from_raw_jwt(jwt)?;
	let is_session_valid = redis::session_exist(&user_jwt.session_id)?;
	let data = json!({
		"valid": is_session_valid
	});

	Ok(data.to_string())
}
