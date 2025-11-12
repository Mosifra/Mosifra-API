use rocket::{Data, data::ToByteUnit, http::Status, serde::json::Json};

use crate::{structs::student::Student, traits::db::Db};

use super::domain::StudentCsvResponse;

#[post("/user/student_csv", data = "<data>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn student_csv(data: Data<'_>) -> Result<Json<StudentCsvResponse>, Status> {
	let data = data.open(2.mebibytes()).into_string().await.map_err(|e| {
		eprintln!("Error while reading data: {e}");
		Status::InternalServerError
	})?;

	let mut reader = csv::Reader::from_reader(data.value.as_bytes());
	for result in reader.records() {
		let record = result.unwrap();
		let student = Student::from_record(record).await?;
		student.insert().await?;
	}
	Ok(Json(StudentCsvResponse { success: true }))
}
