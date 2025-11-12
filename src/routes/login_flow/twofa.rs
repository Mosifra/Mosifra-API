use std::str::FromStr;

use rocket::form::Form;
use serde_json::json;
use uuid::Uuid;

use crate::{
	redis::{self, SessionData, get_user_id_from_twofa},
	structs::{jwt::UserJwt, user_type::UserType},
};

use super::domain::Twofa;

#[post("/twofa", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn twofa(form: Form<Twofa>) -> Result<String, String> {
	let twofa = form.into_inner();

	if redis::check_2fa_code(&twofa)? {
		let session_id = Uuid::new_v4().to_string();
		let session_data = SessionData {
			user_id: get_user_id_from_twofa(&twofa)?,
		};

		let ttl_seconds: u64 = if twofa.remember_me {
			30 * 24 * 3600
		} else {
			30 * 60
		};
		redis::set_session(&session_id, &session_data, ttl_seconds)?;
		redis::invalidate_transactionid(&twofa)?;

		let jwt =
			UserJwt::new_raw_jwt_from_data(session_id, &UserType::from_str(&twofa.user_type)?)?;

		let res = json!({
			"jwt": jwt,
		});

		Ok(res.to_string())
	} else {
		let res = json!({
			"jwt": null,
		});

		Ok(res.to_string())
	}
}
