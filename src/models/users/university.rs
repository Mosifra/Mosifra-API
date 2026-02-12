use rocket::http::Status;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
	error_handling::{StatusOptionHandling, StatusResultHandling},
	models::courses::{Class, CourseType, Internship},
	postgres::Db,
	utils::crypto::{hash_password, verify_password},
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
			.query_opt(
				"SELECT password, id from university WHERE login=$1",
				&[&login],
			)
			.await
			.internal_server_error("SELECT University password error")?;

		let Some(row) = row else {
			return Ok(None);
		};

		let hashed_password: String = row.get(0);

		if verify_password(password, &hashed_password)? {
			let id: String = row.get(1);
			Ok(Some(Self::from_id(id).await?))
		} else {
			Ok(None)
		}
	}

	async fn delete(&self) -> Result<(), Status> {
		let client = Self::setup_database().await?;

		client
			.query_one("DELETE FROM university WHERE id=$1; ", &[&self.id])
			.await
			.internal_server_error("Error during university deletion")?;

		Ok(())
	}
}

impl University {
	pub async fn from_id(id: String) -> Result<Self, Status> {
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

		let class_list = Class::get_classes_from_university_id(id.clone()).await?;

		Ok(Self {
			id,
			login,
			password,
			name,
			mail,
			class_list,
			intership_list: Vec::new(), //WIP
		})
	}

	pub async fn get_classes(&self) -> Result<Vec<Class>, Status> {
		let client = Self::setup_database().await?;

		let query_res = client
			.query("SELECT id FROM class WHERE university_id=$1", &[&self.id])
			.await
			.internal_server_error("Error getting classes")?;

		let mut res = vec![];

		for row in query_res {
			let id = row.get(0);
			res.push(
				Class::from_id(id)
					.await?
					.internal_server_error("No classes found")?,
			);
		}

		Ok(res)
	}

	pub async fn get_course_types(&self) -> Result<Vec<CourseType>, Status> {
		let classes = self.get_classes().await?;
		let mut res = vec![];

		for class in classes {
			res.push(class.course_type);
		}

		Ok(res)
	}

	pub fn has_class(&self, class_id: &str) -> bool {
		for class in &self.class_list {
			if class_id != class.id {
				return false;
			}
		}
		true
	}

	pub async fn get_all() -> Result<Vec<Self>, Status> {
		let client = Self::setup_database().await?;

		let query_res = client
			.query("SELECT id FROM university", &[])
			.await
			.internal_server_error("Error getting universities")?;

		let mut res = vec![];

		for row in query_res {
			let id = row.get(0);
			res.push(Self::from_id(id).await?);
		}

		Ok(res)
	}
}
