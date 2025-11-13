use rocket::http::Status;

use crate::{
	traits::{db::Db, status::StatusResultHandling},
	utils::{hash_password, verify_password},
};

use super::{class::Class, internship::Internship};

#[derive(Debug)]
pub struct University {
	pub id: String,
	pub login: String,
	pub password: String,
	pub name: String,
	pub mail: String,
	pub class_list: Vec<Class>,
	pub intership_list: Vec<Internship>,
}

#[async_trait]
impl Db for University {
	async fn insert(&self) -> Result<(), Status> {
		let client = Self::setup_database().await?;
		let password_hash = hash_password(&self.password)?;

		client
			.query_opt(
				"INSERT INTO university (id, name, mail, login, password) VALUES ($1, $2, $3, $4, $5)",
				&[
					&self.id,
					&self.name,
					&self.mail,
					&self.login,
					&password_hash,
				],
			)
			.await
			.internal_server_error("Error during insert of university")?;

		Ok(())
	}

	async fn login(login: &str, password: &str) -> Result<Option<Self>, Status>
	where
		Self: Sized,
	{
		let client = Self::setup_database().await?;

		let row = client
			.query_opt("SELECT password from university WHERE login=$1", &[&login])
			.await
			.internal_server_error("SELECT University password error")?;

		let Some(row) = row else {
			return Ok(None);
		};

		let hashed_password: String = row.get(0);

		if verify_password(password, &hashed_password)? {
			let row = client
				.query_one(
					"SELECT id, name, login, password, mail from university WHERE login=$1",
					&[&login],
				)
				.await
				.internal_server_error("SELECT University infos error")?;

			let id: String = row.get(0);
			let name: String = row.get(1);
			let login: String = row.get(2);
			let password: String = row.get(3);
			let mail: String = row.get(4);

			let university = Self {
				id,
				login,
				password,
				name,
				mail,
				class_list: vec![],     //WIP
				intership_list: vec![], //WIP
			};
			Ok(Some(university))
		} else {
			Ok(None)
		}
	}
}

impl University {
	pub async fn from_id(id: &str) -> Result<Self, Status> {
		let client = Self::setup_database().await?;

		let row = client
			.query_one(
				"SELECT name, login, password, mail FROM university WHERE id=$1;",
				&[&id],
			)
			.await
			.internal_server_error("SELECT error")?;

		let name: String = row.get(0);
		let login: String = row.get(1);
		let password: String = row.get(2);
		let mail: String = row.get(3);

		Ok(Self {
			id: id.to_string(),
			login,
			password,
			name,
			mail,
			class_list: Vec::new(),     //WIP
			intership_list: Vec::new(), //WIP
		})
	}
}
