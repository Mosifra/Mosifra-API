use chrono::NaiveDate;
use rocket::http::Status;

use anyhow::Result;
use serde::Serialize;
use tokio_postgres::Row;

use crate::{error_handling::StatusResultHandling, postgres::Db};

use super::course_type::CourseType;

#[derive(Debug, Serialize)]
pub struct Internship {
	pub id: String,
	pub course_type: CourseType,
	pub date_start: NaiveDate,
	pub date_end: NaiveDate,
	#[allow(clippy::struct_field_names)] // Normal
	pub min_internship_length: i32,
	#[allow(clippy::struct_field_names)] // Normal
	pub max_internship_length: i32,
	pub title: String,
	pub description: String,
	pub place: String,
}

impl Internship {
	pub async fn insert_with_company(&self, company_id: String) -> Result<(), Status> {
		let client = Self::setup_database().await?;

		client.query(
			"INSERT INTO internship (id, course_type, company_id, start_date, end_date, min_internship_length, max_internship_length, title, description, place) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);",
		&[
					&self.id,
					&self.course_type.to_sql(),
					&company_id,
					&self.date_start,
					&self.date_end,
					&self.min_internship_length,
					&self.max_internship_length,
					&self.title,
					&self.description,
					&self.place
				]
			)
		.await
		.internal_server_error("Failed to insert internship")?;

		Ok(())
	}

	pub async fn insert_with_university(&self, university_id: String) -> Result<(), Status> {
		let client = Self::setup_database().await?;

		client.query(
			"INSERT INTO internship (id, course_type, university_id, start_date, end_date, min_internship_length, max_internship_length, title, description, place) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);",
		&[
					&self.id,
					&self.course_type.to_sql(),
					&university_id,
					&self.date_start,
					&self.date_end,
					&self.min_internship_length,
					&self.max_internship_length,
					&self.title,
					&self.description,
					&self.place
				]
			)
		.await
		.internal_server_error("Failed to insert internship")?;

		Ok(())
	}

	fn from_row(row: &Row) -> Result<Self, Status> {
		let id: String = row.get(0);
		let course_type_id: i32 = row.get(1);
		let course_type = CourseType::from_sql(course_type_id)?;
		let start_date: NaiveDate = row.get(2);
		let end_date: NaiveDate = row.get(3);
		let min_internship_length: i32 = row.get(4);
		let max_internship_length: i32 = row.get(5);
		let title: String = row.get(6);
		let description: String = row.get(7);
		let place: String = row.get(8);

		Ok(Self {
			id,
			course_type,
			date_start: start_date,
			date_end: end_date,
			min_internship_length,
			max_internship_length,
			title,
			description,
			place,
		})
	}

	pub async fn from_company_id(company_id: &str) -> Result<Vec<Self>, Status> {
		let client = Self::setup_database().await?;

		let rows = client
			.query(
				"SELECT id, course_type, start_date, end_date, min_internship_length, max_internship_length, title, description, place from internship WHERE company_id=$1",
				&[&company_id],
			)
			.await
			.internal_server_error("SELECT error")?;

		let mut res = vec![];

		for row in rows {
			res.push(Self::from_row(&row)?);
		}

		Ok(res)
	}

	pub async fn from_university_id(university_id: &str) -> Result<Vec<Self>, Status> {
		let client = Self::setup_database().await?;

		let rows = client
			.query(
				"SELECT id, course_type, start_date, end_date, min_internship_length, max_internship_length, title, description, place from internship WHERE university_id=$1",
				&[&university_id],
			)
			.await
			.internal_server_error("SELECT error")?;

		let mut res = vec![];

		for row in rows {
			res.push(Self::from_row(&row)?);
		}

		Ok(res)
	}

	pub async fn get_all() -> Result<Vec<Self>, Status> {
		let client = Self::setup_database().await?;

		let rows = client
			.query(
				"SELECT id, course_type, start_date, end_date, min_internship_length, max_internship_length, title, description, place from internship",
				&[],
			)
			.await
			.internal_server_error("SELECT error")?;

		let mut res = vec![];

		for row in rows {
			res.push(Self::from_row(&row)?);
		}

		Ok(res)
	}

	pub async fn get_all_based_on_course_types(
		course_types: Vec<CourseType>,
	) -> Result<Vec<Self>, Status> {
		let client = Self::setup_database().await?;
		let mut res = vec![];

		for course_type in course_types {
			let rows = client
			.query(
				"SELECT id, course_type, start_date, end_date, min_internship_length, max_internship_length, title, description, place from internship WHERE course_type=$1",
				&[&course_type.to_sql()],
			)
			.await
			.internal_server_error("SELECT error")?;

			for row in rows {
				res.push(Self::from_row(&row)?);
			}
		}

		Ok(res)
	}
}

#[async_trait]
impl Db for Internship {}
