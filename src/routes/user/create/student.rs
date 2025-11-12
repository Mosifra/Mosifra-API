use rocket::{Data, data::ToByteUnit};
use serde_json::json;

use crate::{structs::student::Student, traits::db::Db};

#[post("/user/student_csv", data = "<data>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn student_csv(data: Data<'_>) -> Result<String, String> {
	let data = data
		.open(2.mebibytes())
		.into_string()
		.await
		.map_err(|e| format!("Error while reading data: {e}"))?;

	let mut reader = csv::Reader::from_reader(data.value.as_bytes());
	for result in reader.records() {
		let record = result.unwrap();
		let student = Student::from_record(record).await?;
		student.insert().await?;
	}
	Ok(json!({"success": true}).to_string())
}
