use tokio_postgres::NoTls;

pub async fn insert_universite(
    nom: String,
    mail: String,
    login: String,
    deuxfa: String,
) -> Result<String, String> {
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL manquant".to_string())?;
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Connexion faille: {e}"))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erreur de connexion: {e}");
        }
    });

    let row = client
        .query_one(
            "INSERT INTO universite (nom), (adresse_mail), (login), (deuxfa_secret) VALUES ($1), ($2), ($3), ($4) RETURNING id",
            &[&nom, &mail, &login, &deuxfa],
        ) //Table université
        .await
        .map_err(|e| format!("Erreur INSERT: {e}"))?;

    let new_id: i32 = row.get(0);
    println!("Université créée avec id = {new_id}");

    Ok(format!(
        "Valeurs {nom}, {mail}, {login}, {deuxfa} insérées avec id {new_id}"
    ))
}
