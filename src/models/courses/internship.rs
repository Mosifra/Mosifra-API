use chrono::NaiveDate;
use rocket::http::Status;

use anyhow::Result;

use crate::{error_handling::StatusResultHandling, postgres::Db};

use super::course_type::CourseType;

#[derive(Debug)]
#[allow(dead_code)] // WIP
pub struct Internship {
	pub id: String,
	pub course_type: CourseType,
	pub date_start: NaiveDate,
	pub date_end: NaiveDate,
	#[allow(clippy::struct_field_names)] // Normal
	pub internship_duration_min_in_weeks: i32,
	#[allow(clippy::struct_field_names)] // Normal
	pub internship_duration_max_in_weeks: i32,
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
					&self.internship_duration_min_in_weeks,
					&self.internship_duration_max_in_weeks,
					&self.title,
					&self.description,
					&self.place
				]
			)
		.await
		.internal_server_error("Failed to insert internship")?;

		Ok(())
	}
}

#[async_trait]
impl Db for Internship {}
