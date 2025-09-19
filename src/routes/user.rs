use rocket::form::Form;
use tokio_postgres::NoTls;

use crate::{
	types::university::{University, UniversityDto},
	utils::{send_2fa_mail, verify_mail},
};

#[post("/user/create_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
pub async fn create_university(form: Form<UniversityDto>) -> Result<String, String> {
    let university = University::try_from(form.into_inner())
        .map_err(|_| "Conversion échouée".to_string())?;
    println!("{university:#?}");

    if verify_mail(&university.mail) {
        println!("correct mail");
    } else {
        println!("incorrect mail");
    }

    let code = send_2fa_mail(university.mail.clone())
        .map_err(|_| "Échec de l’envoi du mail".to_string())?;
    println!("code needs to be {}", code);

    let database_url = std::env::var("DATABASE_URL") //Défini dans le docker-compose si Victor a bien fait son taff
        .map_err(|_| "DATABASE_URL manquant".to_string())?;
    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Connexion faille: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erreur de connexion: {}", e);
        }
    });

    let row = client.query_one(
        "INSERT INTO universite (nom) VALUES ($1) RETURNING id",
        &[&university.name],
    ).await.map_err(|e| format!("Erreur INSERT: {}", e))?;

    let new_id: i32 = row.get(0);
    println!("Université créée avec id = {}", new_id);

    Ok(format!("Université '{}' créée avec id {}", university.name, new_id))
}
