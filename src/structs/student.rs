use uuid::Uuid;

use crate::{
	traits::db::Db,
	utils::{generate_password, hash_password},
};

#[derive(Debug)]
pub struct Student {
	pub id: String,
	pub login: String,
	pub password: String,
	pub mail: String,
	pub first_name: String,
	pub last_name: String,
}

#[derive(Debug, FromForm)]
pub struct StudentDto {
	pub login: String,
	pub mail: String,
	pub first_name: String,
	pub last_name: String,
}

impl TryFrom<StudentDto> for Student {
	type Error = String;

	fn try_from(value: StudentDto) -> Result<Self, Self::Error> {
		let password = generate_password()?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			login: value.login,
			password,
			mail: value.mail,
			first_name: value.first_name,
			last_name: value.last_name,
		})
	}
}

#[async_trait]
impl Db for Student {
	async fn insert(&self) -> Result<String, String> {
		let client = Self::setup_database().await?;
		let password_hash = hash_password(&self.password)?;

		let row = client
        .query_one(
            "INSERT INTO student (first_name, last_name, login, password, mail) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
            &[&self.first_name, &self.last_name, &self.login, &password_hash, &self.mail],
        )
        .await
        .map_err(|e| format!("INSERT Error: {e}"))?;

		let new_id: i32 = row.get(0);
		println!("Student created with id = {new_id}");

		Ok(format!(
			"Values {}, {}, {}, {}, {password_hash} (encoded password) inserted with id {new_id}",
			self.login, self.first_name, self.last_name, self.mail
		))
	}

	async fn get_password_from_mail(mail: &str) -> Result<String, String> {
		let client = Self::setup_database().await?;

		let row = client
			.query_one("SELECT password FROM student WHERE mail=$1;", &[&mail])
			.await
			.map_err(|e| format!("SELECT error: {e}"))?;

		let hashed_password: String = row.get(0);

		Ok(hashed_password)
	}

	async fn get_id_from_mail(_mail: &str) -> Result<String, String> {
		unimplemented!()
	}
}
