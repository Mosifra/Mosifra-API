use uuid::Uuid;

use crate::{
	traits::db::{Db, DbCompany},
	utils::{generate_password, hash_password, verify_password},
};

use super::internship::Internship;

#[derive(Debug)]
pub struct Company {
	pub id: String,
	pub login: String,
	pub password: String,
	pub mail: String,
	pub name: String,
	pub internship_list: Vec<Internship>,
}

#[derive(Debug, FromForm)]
pub struct CompanyDto {
	pub login: String,
	pub mail: String,
	pub name: String,
}

impl TryFrom<CompanyDto> for Company {
	type Error = String;

	fn try_from(value: CompanyDto) -> Result<Self, Self::Error> {
		let password = generate_password()?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			login: value.login,
			password,
			mail: value.mail,
			name: value.name,
			internship_list: Vec::new(),
		})
	}
}

#[async_trait]
impl Db for Company {
	async fn insert(&self) -> Result<String, String> {
		let client = Self::setup_database().await?;
		let password_hash = hash_password(&self.password)?;
		let id = Uuid::new_v4().to_string();

		client
			.query_opt(
				"INSERT INTO company (id, name, login, password, mail) VALUES ($1, $2, $3, $4);",
				&[&id, &self.name, &self.login, &password_hash, &self.mail],
			)
			.await
			.map_err(|e| format!("INSERT error: {e}"))?;

		Ok(format!(
			"Values {}, {}, {}, {password_hash} (encoded password) inserted with id {id}",
			self.login, self.name, self.mail
		))
	}

	async fn login(login: &str, password: &str) -> Result<Self, String>
	where
		Self: Sized,
	{
		let client = Self::setup_database().await?;

		let row = client
			.query_one("SELECT password from company WHERE login=$1", &[&login])
			.await
			.map_err(|e| format!("SELECT error: {e}"))?;

		let hashed_password: String = row.get(0);

		if verify_password(password, &hashed_password)? {
			let row = client
				.query_one(
					"SELECT id, name, login, password, mail from company WHERE login=$1",
					&[&login],
				)
				.await
				.map_err(|e| format!("SELECT error: {e}"))?;

			let id: String = row.get(0);
			let name: String = row.get(1);
			let login: String = row.get(2);
			let password: String = row.get(3);
			let mail: String = row.get(4);

			let company = Self {
				id,
				login,
				password,
				mail,
				name,
				internship_list: vec![], //WIP
			};

			Ok(company)
		} else {
			Err("password incorrect".to_string())
		}
	}
}

#[async_trait]
impl DbCompany for Company {
	async fn get_name(&self, user_id: String) -> Result<String, String> {
		let client = Self::setup_database().await?;

		let row = client
			.query_one("SELECT name FROM company WHERE id=$1;", &[&user_id])
			.await
			.map_err(|e| format!("SELECT error: {e}"))?;

		let res: String = row
			.try_get(0)
			.map_err(|e| format!("Error while trying to get name of company : {e}"))?;
		Ok(res)
	}
}
