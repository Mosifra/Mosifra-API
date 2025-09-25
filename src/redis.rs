use redis::{Connection, TypedCommands};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::login::Twofa;

#[derive(Debug, Serialize, Deserialize)]
struct LoginTransaction {
	user_id: String,
	code: String,
}

fn setup_redis() -> Result<Connection, String> {
	let client = redis::Client::open("redis://default:redis_password@redis/")
		.map_err(|e| format!("Failed to connect to redis : {e}"))?;
	client
		.get_connection()
		.map_err(|e| format!("Failed to get connection : {e}"))
}

pub fn get_transactionid(user_id: String, code: String) -> Result<String, String> {
	let mut con = setup_redis()?;
	let transaction_id = Uuid::new_v4();

	let value = serde_json::to_string(&LoginTransaction { user_id, code })
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
