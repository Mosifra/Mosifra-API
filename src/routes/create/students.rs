use std::io::Cursor;

use rocket::{form::Form, http::Status, serde::json::Json};
use tokio::io::AsyncReadExt;

use crate::{
	error_handling::StatusResultHandling,
	models::{auth::AuthGuard, users::Student},
};

use super::domain::{StudentCsvPayload, StudentCsvResponse};

#[utoipa::path(
	post,
	path = "/create/students",
	request_body = StudentCsvPayload,
	responses(
		(status = 200, description = "Students created successfully", body = StudentCsvResponse),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal Server Error")
	),
	security(
		("api_key" = [])
	),
	tag = "Create"
)]
#[post("/create/students", data = "<student_csv_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_students(
	_auth: AuthGuard,
	student_csv_payload: Form<StudentCsvPayload<'_>>,
) -> Result<Json<StudentCsvResponse>, Status> {
	let payload = student_csv_payload.into_inner();

	let mut reader = payload
		.csv
		.open()
		.await
		.internal_server_error("Failed to open the csv file")?;

	let mut buffer = vec![];
	reader
		.read_to_end(&mut buffer)
		.await
		.internal_server_error("Failed to read csv buffer")?;

	let cursor = Cursor::new(buffer);

	let mut reader = csv::Reader::from_reader(cursor);
	for result in reader.records() {
		let record = result.internal_server_error("Failed to read record")?;
		let student = Student::from_record(record).await?;
		student.insert_self(payload.class.clone()).await?;
	}
	Ok(Json(StudentCsvResponse { success: true }))
}
