use tokio_postgres::{Client, NoTls};

pub async fn setup_database() -> Result<Client, String> {
	let database_url =
		std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL missing".to_string())?;
	let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
		.await
		.map_err(|e| format!("Connection failed: {e}"))?;

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("Connection error: {e}");
		}
	});
	Ok(client)
}

#[async_trait]
pub trait Db {
	#[must_use]
	async fn setup_database() -> Result<Client, String> {
		setup_database().await
	}

	async fn insert(&self) -> Result<String, String>;

	async fn login(login: &str, password: &str) -> Result<Self, String>
	where
		Self: Sized;
}

#[async_trait]
pub trait DbCompany {
	async fn get_name(&self, user_id: String) -> Result<String, String>;
}

pub async fn is_login_taken(username: &str) -> Result<bool, String> {
	let client = setup_database().await?;
	let row = client
		.query_opt("SELECT 1 FROM student WHERE login=$1;", &[&username])
		.await
		.map_err(|e| format!("SELECT error: {e}"))?;

	Ok(row.is_some())
}
