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
pub async fn insert_university(universite: University, twofa: String) -> Result<String, String> {
    let client = setup_database().await?;
    let password_hash = hash_password(&universite.password)?;

    let row = client
        .query_one(
            "INSERT INTO university (name, mail, login, twofa, password) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
            &[&universite.name, &universite.mail, &universite.login, &twofa, &password_hash],
        )
        .await
        .map_err(|e| format!("INSERT error: {e}"))?;

    let new_id: i32 = row.get(0);
    println!("University created with id = {new_id}");

    Ok(format!(
        "Values {}, {}, {}, {twofa}, {password_hash} (encoded password) inserted with id {new_id}",
        universite.name, universite.mail, universite.login
    ))
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_student(student: Student, twofa: String) -> Result<String, String> {
    let client = setup_database().await?;
    let password_hash = hash_password(&student.password)?;

    let row = client
        .query_one(
            "INSERT INTO student (first_name, last_name, login, twofa_secret, password, mail) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id;",
            &[&student.first_name, &student.last_name, &student.login, &twofa, &password_hash, &student.mail],
        )
        .await
        .map_err(|e| format!("INSERT Error: {e}"))?;

    let new_id: i32 = row.get(0);
    println!("Student created with id = {new_id}");

    Ok(format!(
        "Values {}, {}, {}, {}, {twofa}, {password_hash} (encoded password) inserted with id {new_id}",
        student.login, student.first_name, student.last_name, student.mail
    ))
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_company(company: Company, twofa: String) -> Result<String, String> {
    let client = setup_database().await?;
    let password_hash = hash_password(&company.password)?;

    let row = client
        .query_one(
            "INSERT INTO company (name, login, password, twofa, mail) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
            &[&company.name, &company.login, &password_hash, &twofa, &company.mail],
        )
        .await
        .map_err(|e| format!("INSERT error: {e}"))?;

    let new_id: i32 = row.get(0);
    println!("Company created with id = {new_id}");

    Ok(format!(
        "Values {}, {}, {}, {twofa}, {password_hash} (encoded password) inserted with id {new_id}",
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
