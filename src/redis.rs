use redis::{Connection, TypedCommands};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::login_flow::domain::TwofaPayload;

#[derive(Debug, Serialize, Deserialize)]
struct LoginTransaction {
	user_id: String,
	code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
	pub user_id: String,
}

fn setup_redis() -> Result<Connection, Status> {
	let client = redis::Client::open("redis://default:redis_password@redis/").map_err(|e| {
		eprintln!("Error failed to connect to redis : {e}");
		Status::InternalServerError
	})?;
	client.get_connection().map_err(|e| {
		eprintln!("Error failed to get connection : {e}");
		Status::InternalServerError
	})
}

pub fn get_transactionid(user_id: &str, code: String) -> Result<String, Status> {
	let mut con = setup_redis()?;
	let transaction_id = Uuid::new_v4();

	let value = serde_json::to_string(&LoginTransaction {
		user_id: user_id.to_string(),
		code,
	})
	.map_err(|e| {
		eprintln!("Error failed to deserialize LoginTransaction : {e}");
		Status::InternalServerError
	})?;

	con.set_ex(format!("login:{transaction_id}"), value, 900)
		.map_err(|e| {
			eprintln!("Error failed to set login:transaction to redis : {e}");
			Status::InternalServerError
		})?;

	Ok(transaction_id.to_string())
}

pub fn check_2fa_code(twofa: &TwofaPayload) -> Result<bool, Status> {
	let mut con = setup_redis()?;

	let val = con
		.get(format!("login:{}", twofa.transaction_id))
		.map_err(|e| {
			eprintln!("Failed to get login:transaction_id from redis : {e}");
			Status::InternalServerError
		})?
		.unwrap_or_default();

	let check: LoginTransaction = serde_json::from_str(&val).map_err(|e| {
		eprintln!("Failed to deserialize LoginTransaction : {e}");
		Status::InternalServerError
	})?;

	Ok(check.code == twofa.code)
}

pub fn invalidate_transactionid(twofa: &TwofaPayload) -> Result<(), Status> {
	let mut con = setup_redis()?;

	con.del(format!("login:{}", twofa.transaction_id))
		.map_err(|e| {
			eprintln!("Failed to delete login:transaction_id from redis : {e}");
			Status::InternalServerError
		})?;

	Ok(())
}

pub fn get_user_id_from_twofa(twofa: &TwofaPayload) -> Result<String, Status> {
	let mut con = setup_redis()?;

	let val = con
		.get(format!("login:{}", twofa.transaction_id))
		.map_err(|e| {
			eprintln!("Error while trying to get user_id from twofa: {e}");
			Status::InternalServerError
		})?
		.unwrap_or_default();

	if val.is_empty() {
		return Ok(val);
	}

	let check: LoginTransaction = serde_json::from_str(&val).map_err(|e| {
		eprintln!("Error while trying to convert user_id from redis to LoginTransaction: {e}");
		Status::InternalServerError
	})?;

	Ok(check.user_id)
}

pub fn set_session(
	session_id: &str,
	session_data: &SessionData,
	ttl_seconds: u64,
) -> Result<(), Status> {
	let mut con = setup_redis()?;
	let session_data = serde_json::to_string(session_data).map_err(|e| {
		eprintln!("Error during serialization : {e}");
		Status::InternalServerError
	})?;
	con.set_ex(format!("session:{session_id}"), session_data, ttl_seconds)
		.map_err(|e| {
			eprintln!("Failed to set session:session_id to redis : {e}");
			Status::InternalServerError
		})?;

	Ok(())
}

pub fn invalidate_session(session_id: &str) -> Result<(), Status> {
	let mut con = setup_redis()?;

	con.del(format!("session:{session_id}")).map_err(|e| {
		eprintln!("Failed to delete session:session_id from redis : {e}");
		Status::InternalServerError
	})?;

	Ok(())
}

pub fn get_user_id_from_session_id(session_id: String) -> Result<String, Status> {
	let mut con = setup_redis()?;
	let line = con.get(session_id).map_err(|e| {
		eprintln!("Error while getting line : {e}");
		Status::InternalServerError
	})?;
	let Some(line) = line else {
		eprintln!("No volue found to get user_id from session_id");
		return Err(Status::InternalServerError);
	};
	let session_data: SessionData = serde_json::from_str(&line).map_err(|e| {
		eprintln!("Error while deserializing session data: {e}");
		Status::InternalServerError
	})?;

	Ok(session_data.user_id)
}

pub fn session_exist(session_id: &str) -> Result<bool, Status> {
	let mut con = setup_redis()?;

	let res = con.get(format!("session:{session_id}")).map_err(|e| {
		eprintln!("Failed to get session:session_id from redis : {e}");
		Status::InternalServerError
	})?;

	Ok(res.is_some())
}
