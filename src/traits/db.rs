use tokio_postgres::{Client, NoTls};

#[async_trait]
pub trait Db {
	#[must_use]
	async fn setup_database() -> Result<Client, String> {
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

	async fn insert(&self) -> Result<String, String>;

	async fn get_password_from_mail(mail: &str) -> Result<String, String>;

	async fn get_id_from_mail(mail: &str) -> Result<String, String>;
}
