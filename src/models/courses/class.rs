use std::str::FromStr;

use chrono::NaiveDate;
use rocket::http::Status;
use serde::Serialize;
use uuid::Uuid;

use crate::{
	error_handling::{StatusOptionHandling, StatusResultHandling},
	models::{auth::UserJwt, users::University},
	postgres::Db,
	redis,
	routes::create::domain::CreateClassPayload,
};

use super::CourseType;

// For now
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Class {
	pub id: String,
	pub name: String,
	pub course_type: CourseType,
	pub date_internship_start: NaiveDate,
	pub date_internship_end: NaiveDate,
	pub maximum_internship_length: i32,
	pub minimum_internship_length: i32,
	pub university_id: String,
}

impl Class {
	pub async fn from_id(id: String) -> Result<Option<Self>, Status> {
		let client = Self::setup_database().await?;

		let row = client
			.query_opt(
				"SELECT name, course_type, start_date, end_date, min_length, max_length, university_id FROM class WHERE id=$1;",
				&[
					&id,
				],
			)
			.await
			.internal_server_error("Error during class select")?;

		let Some(row) = row else { return Ok(None) };

		let name = row.get(0);
		let course_type_str: String = row.get(1);
		let course_type = CourseType::from_str(&course_type_str)?;
		let date_internship_start = row.get(2);
		let date_internship_end = row.get(3);
		let maximum_internship_length = row.get(4);
		let minimum_internship_length = row.get(5);
		let university_id = row.get(6);

		Ok(Some(Class {
			id,
			name,
			course_type,
			date_internship_start,
			date_internship_end,
			maximum_internship_length,
			minimum_internship_length,
			university_id,
		}))
	}

	pub async fn get_university(&self) -> Result<University, Status> {
		Ok(University::from_id(&self.university_id).await?)
	}
}

impl TryFrom<CreateClassPayload> for Class {
	type Error = Status;

	fn try_from(value: CreateClassPayload) -> Result<Self, Self::Error> {
		let jwt = UserJwt::from_raw_jwt(&value.jwt)?.internal_server_error("Jwt is empty")?;
		let session_id = jwt.session_id;
		let university_id = redis::get_user_id_from_session_id(session_id)?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			name: value.name,
			course_type: value.course_type,
			date_internship_start: value.date_internship_start,
			date_internship_end: value.date_internship_end,
			maximum_internship_length: value.maximum_internship_length,
			minimum_internship_length: value.minimum_internship_length,
			university_id,
		})
	}
}

#[async_trait]
impl Db for Class {
	async fn insert(&self) -> Result<(), Status> {
		let client = Self::setup_database().await?;

		client
			.query_opt(
				"INSERT INTO class (id, name, course_type, start_date, end_date, min_length, max_length, university_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
				&[
					&self.id,
					&self.name,
					&self.course_type.to_sql(),
					&self.date_internship_start,
					&self.date_internship_end,
					&self.maximum_internship_length,
					&self.minimum_internship_length,
					&self.university_id
				],
			)
			.await
			.internal_server_error("Error during class insert")?;

		Ok(())
	}
}
