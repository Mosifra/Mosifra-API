use std::io::Cursor;

use rocket::{form::Form, http::Status, serde::json::Json};
use tokio::io::AsyncReadExt;

use crate::{
	structs::student::Student,
	traits::{db::Db, status::StatusResultHandling},
};

use super::domain::{StudentCsvPayload, StudentCsvResponse};

#[post("/user/student_csv", data = "<student_csv_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn student_csv(
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

	let test = Cursor::new(buffer);

	println!("{}", payload.class);

	let mut reader = csv::Reader::from_reader(test);
	for result in reader.records() {
		let record = result.internal_server_error("Failed to read record")?;
		let student = Student::from_record(record).await?;
		student.insert().await?;
	}
	Ok(Json(StudentCsvResponse { success: true }))
}
