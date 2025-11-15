use redis::{Connection, TypedCommands};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	error_handling::{StatusOptionHandling, StatusResultHandling},
	routes::auth::TwofaPayload,
};

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
	let client = redis::Client::open("redis://default:redis_password@redis/")
		.internal_server_error("Error failed to connect to redis")?;
	client
		.get_connection()
		.internal_server_error("Error failed to get connection")
}

pub fn get_transactionid(user_id: &str, code: String) -> Result<String, Status> {
	let mut con = setup_redis()?;
	let transaction_id = Uuid::new_v4();

	let value = serde_json::to_string(&LoginTransaction {
		user_id: user_id.to_string(),
		code,
	})
	.internal_server_error("Error failed to deserialize LoginTransaction")?;

	con.set_ex(format!("login:{transaction_id}"), value, 900)
		.internal_server_error("Error failed to set login:transaction to redis")?;

	Ok(transaction_id.to_string())
}

pub fn check_2fa_code(twofa: &TwofaPayload) -> Result<bool, Status> {
	let mut con = setup_redis()?;

	let val = con
		.get(format!("login:{}", twofa.transaction_id))
		.internal_server_error("Failed to get login:transaction_id from redis")?
		.internal_server_error("Transaction id is empty")?;

	let check: LoginTransaction = serde_json::from_str(&val)
		.internal_server_error("Failed to deserialize LoginTransaction")?;

	Ok(check.code == twofa.code)
}

pub fn invalidate_transactionid(twofa: &TwofaPayload) -> Result<(), Status> {
	let mut con = setup_redis()?;

	con.del(format!("login:{}", twofa.transaction_id))
		.internal_server_error("Failed to delete login:transaction_id from redis")?;

	Ok(())
}

pub fn get_user_id_from_twofa(twofa: &TwofaPayload) -> Result<String, Status> {
	let mut con = setup_redis()?;

	let val = con
		.get(format!("login:{}", twofa.transaction_id))
		.internal_server_error("Error while trying to get user_id from twofa")?
		.internal_server_error("Transaction id is empty")?;

	if val.is_empty() {
		return Ok(val);
	}

	let check: LoginTransaction = serde_json::from_str(&val).internal_server_error(
		"Error while trying to convert user_id from redis to LoginTransaction",
	)?;

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
		.internal_server_error("Failed to set session:session_id to redis")?;

	Ok(())
}

pub fn invalidate_session(session_id: &str) -> Result<(), Status> {
	let mut con = setup_redis()?;

	con.del(format!("session:{session_id}"))
		.internal_server_error("Failed to delete session:session_id from redis")?;

	Ok(())
}

pub fn get_user_id_from_session_id(session_id: String) -> Result<String, Status> {
	let mut con = setup_redis()?;
	let line = con
		.get(session_id)
		.internal_server_error("Error while getting line")?;
	let line = line.internal_server_error("No value found to get user_id from session_id")?;
	let session_data: SessionData = serde_json::from_str(&line)
		.internal_server_error("Error while deserializing session data")?;

	Ok(session_data.user_id)
}

pub fn session_exist(session_id: &str) -> Result<bool, Status> {
	let mut con = setup_redis()?;

	let res = con
		.get(format!("session:{session_id}"))
		.internal_server_error("Failed to get session:session_id from redis")?;

	Ok(res.is_some())
}
