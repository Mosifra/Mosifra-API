use tokio_postgres::{Client, NoTls};

use crate::{
    types::{company::Company, student::Student, university::University},
    utils::hash_password,
};

async fn setup_database() -> Result<Client, String> {
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL missing".to_string())?;
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Connection failed: {e}"))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {e}");
        }
    });
    Ok(client)
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_university(university: University) -> Result<String, String> {
    let client = setup_database().await?;
    let password_hash = hash_password(&university.password)?;

    let row = client
        .query_one(
            "INSERT INTO university (name, mail, login, twofa, password) VALUES ($1, $2, $3, $4) RETURNING id;",
            &[&university.name, &university.mail, &university.login, &password_hash],
        )
        .await
        .map_err(|e| format!("INSERT error: {e}"))?;

    let new_id: i32 = row.get(0);
    println!("University created with id = {new_id}");

    Ok(format!(
        "Values {}, {}, {}, {password_hash} (encoded password) inserted with id {new_id}",
        university.name, university.mail, university.login
    ))
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_student(student: Student) -> Result<String, String> {
    let client = setup_database().await?;
    let password_hash = hash_password(&student.password)?;

    let row = client
        .query_one(
            "INSERT INTO student (first_name, last_name, login, password, mail) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
            &[&student.first_name, &student.last_name, &student.login, &password_hash, &student.mail],
        )
        .await
        .map_err(|e| format!("INSERT Error: {e}"))?;

    let new_id: i32 = row.get(0);
    println!("Student created with id = {new_id}");

    Ok(format!(
        "Values {}, {}, {}, {}, {password_hash} (encoded password) inserted with id {new_id}",
        student.login, student.first_name, student.last_name, student.mail
    ))
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_company(company: Company) -> Result<String, String> {
    let client = setup_database().await?;
    let password_hash = hash_password(&company.password)?;

    let row = client
        .query_one(
            "INSERT INTO company (name, login, password, mail) VALUES ($1, $2, $3, $4) RETURNING id;",
            &[&company.name, &company.login, &password_hash, &company.mail],
        )
        .await
        .map_err(|e| format!("INSERT error: {e}"))?;

    let new_id: i32 = row.get(0);
    println!("Company created with id = {new_id}");

    Ok(format!(
        "Values {}, {}, {}, {password_hash} (encoded password) inserted with id {new_id}",
        company.login, company.name, company.mail
    ))
}

#[allow(clippy::missing_errors_doc)]
pub async fn get_university_password_from_mail(mail: &str) -> Result<String, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("SELECT password FROM university WHERE mail=$1;", &[&mail])
        .await
        .map_err(|e| format!("SELECT error: {e}"))?;

    let hashed_password: String = row.get(0);

    Ok(hashed_password)
}

#[allow(clippy::missing_errors_doc)]
pub async fn get_company_password_from_mail(mail: &str) -> Result<String, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("SELECT password FROM company WHERE mail=$1;", &[&mail])
        .await
        .map_err(|e| format!("SELECT error: {e}"))?;

    let hashed_password: String = row.get(0);

    Ok(hashed_password)
}

#[allow(clippy::missing_errors_doc)]
pub async fn get_student_password_from_mail(mail: &str) -> Result<String, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("SELECT password FROM student WHERE mail=$1;", &[&mail])
        .await
        .map_err(|e| format!("SELECT error: {e}"))?;

    let hashed_password: String = row.get(0);

    Ok(hashed_password)
}

#[allow(clippy::missing_errors_doc)]
pub async fn is_2fa_null_for_university(id: &str) -> Result<bool, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("SELECT twofa FROM university WHERE id=$1;", &[&id])
        .await
        .map_err(|e| format!("SELECT error: {e}"))?;

    let row_result: String = row.get(0);
    if row_result == "NULL" {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[allow(clippy::missing_errors_doc)]
pub async fn is_2fa_null_for_company(id: &str) -> Result<bool, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("SELECT twofa FROM company WHERE id=$1;", &[&id])
        .await
        .map_err(|e| format!("SELECT error: {e}"))?;

    let row_result: String = row.get(0);
    if row_result == "NULL" {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[allow(clippy::missing_errors_doc)]
pub async fn is_2fa_null_for_student(id: &str) -> Result<bool, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("SELECT twofa FROM student WHERE id=$1;", &[&id])
        .await
        .map_err(|e| format!("SELECT error: {e}"))?;

    let row_result: String = row.get(0);
    if row_result == "NULL" {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_2fa_for_student(id: &str, twofa: String) -> Result<String, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("INSERT INTO student (twofa) VALUES ($1)", &[&twofa])
        .await
        .map_err(|e| format!("INSERT error: {e}"))?;

    Ok("Inséré".to_string())
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_2fa_for_company(id: &str, twofa: String) -> Result<String, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("INSERT INTO company (twofa) VALUES ($1)", &[&twofa])
        .await
        .map_err(|e| format!("INSERT error: {e}"))?;

    Ok("Inséré".to_string())
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_2fa_for_university(id: &str, twofa: String) -> Result<String, String> {
    let client = setup_database().await?;

    let row = client
        .query_one("INSERT INTO university (twofa) VALUES ($1)", &[&twofa])
        .await
        .map_err(|e| format!("INSERT error: {e}"))?;

    Ok("Inséré".to_string())
}