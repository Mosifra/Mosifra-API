use rocket::http::Status;
use tokio_postgres::{Client, NoTls};

use crate::error_handling::StatusResultHandling;

pub async fn setup_database() -> Result<Client, Status> {
	let database_url =
		std::env::var("DATABASE_URL").internal_server_error("DATABASE_URL missing")?;
	let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
		.await
		.internal_server_error("Error connection to prostgres failed")?;

	tokio::spawn(async move {
		let _ = connection.await.internal_server_error("Connection error");
	});
	Ok(client)
}

#[async_trait]
pub trait Db {
	#[must_use]
	async fn setup_database() -> Result<Client, Status> {
		setup_database().await
	}

	async fn insert(&self) -> Result<(), Status> {
		unimplemented!("Need to refacto this system")
	}

	#[allow(unused_variables)]
	async fn login(login: &str, password: &str) -> Result<Option<Self>, Status>
	where
		Self: Sized,
	{
		unimplemented!("Only for structs that represents users")
	}

	#[allow(unused_variables)]
	async fn get_name(&self, user_id: String) -> Result<String, Status> {
		unimplemented!("For Company struct only")
	}

	async fn delete(&self) -> Result<(), Status> {
		unimplemented!("Need implementation")
	}
}

pub async fn is_login_taken(username: &str) -> Result<bool, Status> {
	let client = setup_database().await?;
	let row = client
		.query_opt("SELECT 1 FROM student WHERE login=$1;", &[&username])
		.await
		.internal_server_error("Error during selection of login to check if login is taken")?;

	Ok(row.is_some())
}
