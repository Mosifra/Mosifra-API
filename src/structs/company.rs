use uuid::Uuid;

use crate::{
    traits::db::Db,
    utils::{generate_password, hash_password},
};

use super::internship::Internship;

#[derive(Debug)]
pub struct Company {
    pub id: String,
    pub login: String,
    pub password: String,
    pub mail: String,
    pub name: String,
    pub internship_list: Vec<Internship>,
}

#[derive(Debug, FromForm)]
pub struct CompanyDto {
    pub login: String,
    pub mail: String,
    pub name: String,
}

impl TryFrom<CompanyDto> for Company {
    type Error = String;

    fn try_from(value: CompanyDto) -> Result<Self, Self::Error> {
        let password = generate_password()?;

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            login: value.login,
            password,
            mail: value.mail,
            name: value.name,
            internship_list: Vec::new(),
        })
    }
}

#[async_trait]
impl Db for Company {
    async fn insert(&self) -> Result<String, String> {
        let client = Self::setup_database().await?;
        let password_hash = hash_password(&self.password)?;

        let row = client
			.query_one(
				"INSERT INTO company (name, login, password, mail) VALUES ($1, $2, $3, $4) RETURNING id;",
				&[&self.name, &self.login, &password_hash, &self.mail],
			)
			.await
			.map_err(|e| format!("INSERT error: {e}"))?;

        let new_id: i32 = row.get(0);
        println!("Company created with id = {new_id}");

        Ok(format!(
            "Values {}, {}, {}, {password_hash} (encoded password) inserted with id {new_id}",
            self.login, self.name, self.mail
        ))
    }

    async fn get_password_from_mail(mail: &str) -> Result<String, String> {
        let client = Self::setup_database().await?;

        let row = client
            .query_one("SELECT password FROM company WHERE mail=$1;", &[&mail])
            .await
            .map_err(|e| format!("SELECT error: {e}"))?;

        let hashed_password: String = row.get(0);

        Ok(hashed_password)
    }

    async fn get_id_from_mail(_mail: &str) -> Result<String, String> {
        unimplemented!()
    }

    // async fn get_name_from_userid(user_id: String) -> Result<String, String> {
    //     let client = Self::setup_database().await?;
    //
    //     let row = client
    //         .query_one("SELECT name FROM company WHERE id=$1;", &[&user_id])
    //         .await
    //         .map_err(|e| format!("SELECT error: {e}"))?;
    //     Ok(String::from("a"))
    // }
}
