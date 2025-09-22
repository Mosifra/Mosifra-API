use tokio_postgres::{Client, NoTls};

use crate::{
	types::{company::Company, student::Student, university::University},
	utils::hash_password,
};

async fn setup_database() -> Result<Client, String> {
	let database_url =
		std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL manquant".to_string())?;
	let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
		.await
		.map_err(|e| format!("Connexion failed: {e}"))?;

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("Erreur de connexion: {e}");
		}
	});
	Ok(client)
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_universite(universite: University, deuxfa: String) -> Result<String, String> {
	let client = setup_database().await?;
	let password_hash = hash_password(&universite.password)?;

	let row = client
        .query_one(
            "INSERT INTO universite (nom, adresse_mail, login, deuxfa_secret, mot_de_passe) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
            &[&universite.name, &universite.mail, &universite.login, &deuxfa, &password_hash],
        )
        .await
        .map_err(|e| format!("Erreur INSERT: {e}"))?;

	let new_id: i32 = row.get(0);
	println!("Université créée avec id = {new_id}");

	Ok(format!(
		"Valeurs {}, {}, {}, {deuxfa}, {password_hash} (mot de passe encodé) insérées avec id {new_id}",
		universite.name, universite.mail, universite.login
	))
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_student(student: Student, deuxfa: String) -> Result<String, String> {
	let client = setup_database().await?;
	let password_hash = hash_password(&student.password)?;

	let row = client
        .query_one(
            "INSERT INTO etudiant (nom, prenom, login, deuxfa_secret, mot_de_passe, adresse_mail) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id;",
            &[&student.first_name, &student.last_name, &student.login, &deuxfa, &password_hash, &student.mail],
        )
        .await
        .map_err(|e| format!("Erreur INSERT: {e}"))?;

	let new_id: i32 = row.get(0);
	println!("Etudiant crée avec id = {new_id}");

	Ok(format!(
		"Valeurs {}, {}, {}, {}, {deuxfa}, {password_hash} (mot de passe encodé) insérées avec id {new_id}",
		student.login, student.first_name, student.last_name, student.mail
	))
}

#[allow(clippy::missing_errors_doc)]
pub async fn insert_company(company: Company, deuxfa: String) -> Result<String, String> {
	let client = setup_database().await?;
	let password_hash = hash_password(&company.password)?;

	let row = client
        .query_one(
            "INSERT INTO entreprise (nom, login, mot_de_passe, deuxfa_secret, adresse_mail) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
            &[&company.name, &company.login, &password_hash, &deuxfa, &company.mail],
        )
        .await
        .map_err(|e| format!("Erreur INSERT: {e}"))?;

	let new_id: i32 = row.get(0);
	println!("Company créée avec id = {new_id}");

	Ok(format!(
		"Valeurs {}, {}, {}, {deuxfa}, {password_hash} (mot de passe encodé) insérées avec id {new_id}",
		company.login, company.name, company.mail
	))
}

#[allow(clippy::missing_errors_doc)]
pub async fn get_university_password_from_mail(mail: &str) -> Result<String, String> {
	let client = setup_database().await?;

	let row = client
		.query_one(
			"SELECT mot_de_passe FROM universite WHERE adresse_mail=$1;",
			&[&mail],
		)
		.await
		.map_err(|e| format!("Erreur SELECT: {e}"))?;

	let hashed_password: String = row.get(0);

	Ok(hashed_password)
}
