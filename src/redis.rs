use redis::{Connection, TypedCommands};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::login::Twofa;

#[derive(Debug, Serialize, Deserialize)]
struct LoginTransaction {
	user_id: String,
	code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
	pub user_id: String,
	pub user_type: String,
}

fn setup_redis() -> Result<Connection, String> {
	let client = redis::Client::open("redis://default:redis_password@redis/")
		.map_err(|e| format!("Failed to connect to redis : {e}"))?;
	client
		.get_connection()
		.map_err(|e| format!("Failed to get connection : {e}"))
}

pub fn get_transactionid(user_id: &str, code: String) -> Result<String, String> {
	let mut con = setup_redis()?;
	let transaction_id = Uuid::new_v4();

	let value = serde_json::to_string(&LoginTransaction {
		user_id: user_id.to_string(),
		code,
	})
	.map_err(|e| format!("Failed to deserialize LoginTransaction : {e}"))?;

	con.set_ex(format!("login:{transaction_id}"), value, 900)
		.map_err(|e| format!("Failed to set login:transaction to redis : {e}"))?;

	Ok(transaction_id.to_string())
}

pub fn check_2fa_code(twofa: &Twofa) -> Result<bool, String> {
	let mut con = setup_redis()?;

	let val = con
		.get(format!("login:{}", twofa.transaction_id))
		.map_err(|e| format!("Failed to get login:transaction_id from redis : {e}"))?
		.unwrap_or_default();

	let check: LoginTransaction = serde_json::from_str(&val)
		.map_err(|e| format!("Failed to deserialize LoginTransaction : {e}"))?;

	Ok(check.code == twofa.code)
}

pub fn invalidate_transactionid(twofa: &Twofa) -> Result<(), String> {
	let mut con = setup_redis()?;

	con.del(format!("login:{}", twofa.transaction_id))
		.map_err(|e| format!("Failed to delete login:transaction_id from redis : {e}"))?;

	Ok(())
}

pub fn get_user_id_from_twofa(twofa: &Twofa) -> Result<String, String> {
	let mut con = setup_redis()?;

	let val = con
		.get(format!("login:{}", twofa.transaction_id))
		.unwrap()
		.unwrap();

	let check: LoginTransaction = serde_json::from_str(&val).unwrap();

	Ok(check.user_id)
}

pub fn set_session(
	session_id: &str,
	session_data: &SessionData,
	ttl_seconds: u64,
) -> Result<(), String> {
	let mut con = setup_redis()?;
	let session_data = serde_json::to_string(session_data)
		.map_err(|e| format!("Error during serialization : {e}"))?;
	con.set_ex(format!("session:{session_id}"), session_data, ttl_seconds)
		.map_err(|e| format!("Failed to set session:session_id to redis : {e}"))?;

	Ok(())
}

pub fn invalidate_session(session_id: &str) -> Result<(), String> {
	let mut con = setup_redis()?;

	con.del(format!("session:{session_id}"))
		.map_err(|e| format!("Failed to delete session:session_id from redis : {e}"))?;

	Ok(())
}

pub async fn get_user_id_from_session_id(session_id: String) -> Result<String, String> {
	let mut con = setup_redis()?;
	let line = con
		.get(session_id)
		.map_err(|e| format!("Error while getting line : {e}"))?;
	let line = match line {
		Some(line) => line,
		None => return Err(format!("No value found")),
	};
	let session_data: SessionData = serde_json::from_str(&line)
		.map_err(|e| format!("Error while deserializing session data: {e}"))?;

	Ok(session_data.user_id.to_string())
}
