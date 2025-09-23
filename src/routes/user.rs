use rocket::form::Form;

use crate::{
    db::{insert_company, insert_student, insert_university},
    types::{
        company::{Company, CompanyDto},
        student::{Student, StudentDto},
        university::{University, UniversityDto},
    },
    utils::verify_mail,
};

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

    insert_university(university).await
}

#[post("/user/create_student", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_student(form: Form<StudentDto>) -> Result<String, String> {
    let student = Student::try_from(form.into_inner())?;
    println!("{student:#?}");

    if verify_mail(&student.mail) {
        println!("correct mail");
    } else {
        println!("incorrect mail");
    }

    insert_student(student).await
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

    insert_company(company).await
}
