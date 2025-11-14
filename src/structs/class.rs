use chrono::NaiveDate;
use rocket::http::Status;
use uuid::Uuid;

use crate::{
	redis,
	routes::user::create::domain::CreateClassPayload,
	traits::{
		db::Db,
		status::{StatusOptionHandling, StatusResultHandling},
	},
};

use super::{course_type::CourseType, jwt::UserJwt, university};

// For now
#[allow(dead_code)]
#[derive(Debug)]
pub struct Class {
	id: String,
	name: String,
	course_type: CourseType,
	date_internship_start: NaiveDate,
	date_internship_end: NaiveDate,
	maximum_internship_length: i32,
	minimum_internship_length: i32,
	university_id: String,
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
