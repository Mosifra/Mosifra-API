use chrono::NaiveDate;
use rocket::http::Status;
use serde::Serialize;
use uuid::Uuid;

use crate::{
	error_handling::{StatusOptionHandling, StatusResultHandling},
	models::users::University,
	postgres::Db,
	redis,
	routes::create::domain::CreateClassPayload,
};

use super::CourseType;

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
		let course_type_str: i32 = row.get(1);
		let course_type = CourseType::from_sql(course_type_str)?;
		let date_internship_start = row.get(2);
		let date_internship_end = row.get(3);
		let maximum_internship_length = row.get(4);
		let minimum_internship_length = row.get(5);
		let university_id = row.get(6);

		Ok(Some(Self {
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
		University::from_id(&self.university_id).await
	}

	pub fn try_from_payload(value: CreateClassPayload, session_id: String) -> Result<Self, Status> {
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

	pub async fn get_classes_from_university_id(
		university_id: String,
	) -> Result<Vec<Self>, Status> {
		let client = Self::setup_database().await?;

		let query_res = client
			.query("SELECT id FROM class WHERE university_id=$1", &[&university_id])
			.await
			.internal_server_error("Error getting classes")?;

		let mut res = vec![];

		for row in query_res {
			let id = row.get(0);
			res.push(
				Self::from_id(id)
					.await?
					.internal_server_error("No classes found")?,
			);
		}

		Ok(res)
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
