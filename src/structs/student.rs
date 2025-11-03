use csv::StringRecord;
use uuid::Uuid;

use crate::{
	traits::db::Db,
	utils::{self, generate_login, generate_password, hash_password},
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
	pub mail: String,
	pub first_name: String,
	pub last_name: String,
}

impl Student {
	pub async fn try_from(value: StudentDto) -> Result<Self, String> {
		let password = generate_password()?;
		let login = generate_login(&value.first_name, &value.last_name).await?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			login,
			password,
			mail: value.mail,
			first_name: value.first_name,
			last_name: value.last_name,
		})
	}

	pub async fn from_record(record: StringRecord) -> Result<Self, String> {
		let first_name = (&record[0]).to_string();
		let last_name = (&record[1]).to_string();

		let id = Uuid::new_v4().to_string();
		let login = generate_login(&first_name, &last_name).await?;
		let password = generate_password()?;
		let mail = record[2].to_string();

		Ok(Self {
			id,
			login,
			password,
			mail,
			first_name,
			last_name,
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
