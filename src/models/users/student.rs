use csv::StringRecord;
use rocket::http::Status;
use uuid::Uuid;

use crate::{
	error_handling::{StatusOptionHandling, StatusResultHandling},
	postgres::{Db, is_login_taken},
	utils::crypto::{generate_password, hash_password, verify_password},
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

impl Student {
	pub async fn from_record(record: StringRecord) -> Result<Self, Status> {
		let first_name = record[0].to_string();
		let last_name = record[1].to_string();

		let id = Uuid::new_v4().to_string();
		let login = generate_login(&first_name, &last_name).await?;
		let password = generate_password()?;
		let mail = record[2].to_string();

		let student = Self {
			id,
			login,
			password,
			mail,
			first_name,
			last_name,
		};

		println!("{student:#?}");

		Ok(student)
	}
}

#[async_trait]
impl Db for Student {
	async fn insert(&self) -> Result<(), Status> {
		let client = Self::setup_database().await?;
		let password_hash = hash_password(&self.password)?;
		let id = Uuid::new_v4().to_string();

		client
        .query_opt(
            "INSERT INTO student (id, first_name, last_name, login, password, mail) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&id, &self.first_name, &self.last_name, &self.login, &password_hash, &self.mail],
        )
        .await
        .internal_server_error("INSERT student Error")?;

		Ok(())
	}

	async fn login(login: &str, password: &str) -> Result<Option<Self>, Status>
	where
		Self: Sized,
	{
		let client = Self::setup_database().await?;

		let row = client
			.query_one("SELECT password from student WHERE login=$1", &[&login])
			.await
			.internal_server_error("SELECT error")?;

		let hashed_password: String = row.get(0);

		if verify_password(password, &hashed_password)? {
			let row = client
				.query_one(
					"SELECT id, first_name, last_name, login, password, mail from student WHERE login=$1",
					&[&login],
				)
				.await
				.internal_server_error("SELECT error")?;

			let id: String = row.get(0);
			let first_name: String = row.get(1);
			let last_name: String = row.get(2);
			let login: String = row.get(3);
			let password: String = row.get(4);
			let mail: String = row.get(5);

			let student = Self {
				id,
				login,
				password,
				mail,
				first_name,
				last_name,
			};

			Ok(Some(student))
		} else {
			Ok(None)
		}
	}
}

// Yaniss Lasbordes -> ylasbordes1 if already exist ylasbordes2 until ylasbordes{n}

pub async fn generate_login(first_name: &str, last_name: &str) -> Result<String, Status> {
	let first_name = first_name.to_lowercase();
	let last_name = last_name.to_lowercase();
	let first_name_letter = first_name
		.chars()
		.next()
		.internal_server_error("Login generation error : login is empty")?;
	let mut res;
	let mut i = 1;

	loop {
		res = format!("{first_name_letter}{last_name}{i}");
		if !is_login_taken(&res).await? {
			break;
		}
		i += 1;
	}

	Ok(res)
}
