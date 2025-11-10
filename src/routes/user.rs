use rocket::{Data, data::ToByteUnit, form::Form};

use crate::{
	structs::{
		company::{Company, CompanyDto},
		student::Student,
		university::{University, UniversityDto},
	},
	traits::db::Db,
	utils::verify_mail,
};

use serde_json::json;

#[post("/user/create_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_university(form: Form<UniversityDto>) -> Result<String, String> {
	let university = University::try_from(form.into_inner())
		.map_err(|()| "Error while converting UniversityDto".to_string())?; // Not ideal but will do
	println!("{university:#?}");

	if verify_mail(&university.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	university.insert().await
}

#[post("/user/create_company", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_company(form: Form<CompanyDto>) -> Result<String, String> {
	let company = Company::try_from(form.into_inner())?;
	println!("{company:#?}");

	if verify_mail(&company.mail) {
		println!("correct mail");
	} else {
		println!("incorrect mail");
	}

	company.insert().await
}

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
