use csv::StringRecord;
use rocket::http::Status;
use uuid::Uuid;

use crate::{
	error_handling::{StatusOptionHandling, StatusResultHandling},
	models::courses::Class,
	postgres::{Db, is_login_taken},
	utils::crypto::{generate_password, hash_password, verify_password},
};

use any_ascii::any_ascii;

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
	pub async fn from_id(user_id: String) -> Result<Self, Status> {
		let client = Self::setup_database().await?;

		let row = client
			.query_one(
				"SELECT first_name, last_name, login, password, mail from student WHERE id=$1",
				&[&user_id],
			)
			.await
			.internal_server_error("SELECT error")?;

		let first_name: String = row.get(0);
		let last_name: String = row.get(1);
		let login: String = row.get(2);
		let password: String = row.get(3);
		let mail: String = row.get(4);

		let student = Self {
			id: user_id,
			login,
			password,
			mail,
			first_name,
			last_name,
		};

		Ok(student)
	}

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

	pub async fn get_class(&self) -> Result<Option<Class>, Status> {
		let client = Self::setup_database().await?;

		let row = client
			.query_opt("SELECT class_id from student WHERE id=$1", &[&self.id])
			.await
			.internal_server_error("SELECT error")?;

		let Some(row) = row else { return Ok(None) };

		let class_id: String = row.get(0);

		let class = Class::from_id(class_id).await?;

		Ok(class)
	}

	pub async fn insert_self(&self, class_id: String) -> Result<(), Status> {
		let client = Self::setup_database().await?;
		let password_hash = hash_password(&self.password)?;
		let id = Uuid::new_v4().to_string();

		client
        .query_opt(
            "INSERT INTO student (id, first_name, last_name, login, password, mail, class_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[&id, &self.first_name, &self.last_name, &self.login, &password_hash, &self.mail, &class_id],
        )
        .await
        .internal_server_error("INSERT student Error")?;

		Ok(())
	}
}

#[async_trait]
impl Db for Student {
	async fn insert(&self) -> Result<(), Status> {
		unimplemented!()
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
	let last_name = any_ascii(&last_name.to_lowercase()).replace([' ', '-'], "");
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
