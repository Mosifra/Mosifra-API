use rocket::http::Status;
use uuid::Uuid;

use crate::{
	error_handling::StatusResultHandling,
	models::courses::Internship,
	postgres::Db,
	utils::crypto::{hash_password, verify_password},
};

#[derive(Debug)]
pub struct Company {
	pub id: String,
	pub login: String,
	pub password: String,
	pub mail: String,
	pub name: String,
	pub internship_list: Vec<Internship>,
}

#[async_trait]
impl Db for Company {
	async fn insert(&self) -> Result<(), Status> {
		let client = Self::setup_database().await?;
		let password_hash = hash_password(&self.password)?;
		let id = Uuid::new_v4().to_string();

		client
			.query_opt(
				"INSERT INTO company (id, name, login, password, mail) VALUES ($1, $2, $3, $4, $5);",
				&[&id, &self.name, &self.login, &password_hash, &self.mail],
			)
			.await
			.internal_server_error("Error during company insert")?;

		println!("==========DEBUG==========");
		println!("login : {}", self.login);
		println!("password : {}", self.password);
		println!("==========DEBUG==========");

		Ok(())
	}

	async fn login(login: &str, password: &str) -> Result<Option<Self>, Status>
	where
		Self: Sized,
	{
		let client = Self::setup_database().await?;

		let row = client
			.query_one("SELECT password from company WHERE login=$1", &[&login])
			.await
			.internal_server_error("SELECT error")?;

		let hashed_password: String = row.get(0);

		if verify_password(password, &hashed_password)? {
			let row = client
				.query_one(
					"SELECT id, name, login, password, mail from company WHERE login=$1",
					&[&login],
				)
				.await
				.internal_server_error("SELECT error")?;

			let id: String = row.get(0);
			let name: String = row.get(1);
			let login: String = row.get(2);
			let password: String = row.get(3);
			let mail: String = row.get(4);
			let internship_list = Internship::from_company_id(&id).await?;

			let company = Self {
				id,
				login,
				password,
				mail,
				name,
				internship_list,
			};

			Ok(Some(company))
		} else {
			Ok(None)
		}
	}

	async fn get_name(&self, user_id: String) -> Result<String, Status> {
		let client = Self::setup_database().await?;

		let row = client
			.query_one("SELECT name FROM company WHERE id=$1;", &[&user_id])
			.await
			.internal_server_error("SELECT error")?;

		let res: String = row
			.try_get(0)
			.internal_server_error("Error while trying to get name of company")?;
		Ok(res)
	}
}
