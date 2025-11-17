use std::str::FromStr;

use rocket::{http::Status, serde::json::Json};
use uuid::Uuid;

use crate::{
	error_handling::StatusOptionHandling,
	models::auth::{AuthGuard, UserType},
	redis::{
		SessionData, check_2fa_code, get_user_id_from_twofa, invalidate_transactionid, set_session,
	},
};

use super::domain::{TwofaPayload, TwofaResponse};

#[post("/auth/twofa", data = "<twofa_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn twofa(twofa_payload: Json<TwofaPayload>) -> Result<Json<TwofaResponse>, Status> {
	let twofa = twofa_payload.into_inner();

	if check_2fa_code(&twofa)? {
		let session_id = Uuid::new_v4().to_string();
		let user_id = get_user_id_from_twofa(&twofa)?;
		if user_id.is_empty() {
			return Ok(Json(TwofaResponse {
				valid: false,
				jwt: None,
			}));
		}
		let session_data = SessionData { user_id };

		let ttl_seconds: u64 = if twofa.remember_me {
			30 * 24 * 3600
		} else {
			30 * 60
		};
		set_session(&session_id, &session_data, ttl_seconds)?;
		invalidate_transactionid(&twofa)?;

		let jwt =
			AuthGuard::new_raw_jwt_from_data(session_id, &UserType::from_str(&twofa.user_type)?)?
				.internal_server_error("JWT is somehow not valid")?;

		Ok(Json(TwofaResponse {
			valid: true,
			jwt: Some(jwt),
		}))
	} else {
		Ok(Json(TwofaResponse {
			valid: false,
			jwt: None,
		}))
	}
}
