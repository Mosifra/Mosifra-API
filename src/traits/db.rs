use rocket::http::Status;
use tokio_postgres::{Client, NoTls};

pub async fn setup_database() -> Result<Client, Status> {
	let database_url = std::env::var("DATABASE_URL").map_err(|_| {
		eprintln!("DATABASE_URL missing");
		Status::InternalServerError
	})?;
	let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
		.await
		.map_err(|e| {
			eprintln!("Error connection to prostgres failed: {e}");
			Status::InternalServerError
		})?;

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
	async fn setup_database() -> Result<Client, Status> {
		setup_database().await
	}

	async fn insert(&self) -> Result<(), Status>;

	async fn login(login: &str, password: &str) -> Result<Option<Self>, Status>
	where
		Self: Sized;
}

#[async_trait]
pub trait DbCompany {
	async fn get_name(&self, user_id: String) -> Result<String, Status>;
}

pub async fn is_login_taken(username: &str) -> Result<bool, Status> {
	let client = setup_database().await?;
	let row = client
		.query_opt("SELECT 1 FROM student WHERE login=$1;", &[&username])
		.await
		.map_err(|e| {
			eprintln!("Error during selection of login to check if login is taken : {e}");
			Status::InternalServerError
		})?;

	Ok(row.is_some())
}
