use uuid::Uuid;

use crate::{
	traits::db::Db,
	utils::{generate_password, hash_password, verify_password},
};

use super::{
	class::{Class, ClassDto, TryFromVecClassDtoToClassVec},
	internship::{Internship, InternshipDto, TryFromVecInternshipDtoToInternshipVec},
};

#[derive(Debug)]
pub struct University {
	pub id: String,
	pub login: String,
	pub password: String,
	pub name: String,
	pub mail: String,
	pub class_list: Vec<Class>,
	pub intership_list: Vec<Internship>,
}

#[derive(Debug, FromForm)]
pub struct UniversityDto {
	pub login: String,
	pub name: String,
	pub mail: String,
	pub class_list: Vec<ClassDto>,
	pub intership_list: Vec<InternshipDto>,
}

impl TryFrom<UniversityDto> for University {
	type Error = ();

	fn try_from(value: UniversityDto) -> Result<Self, Self::Error> {
		let class_list = Vec::<Class>::try_from_classdto_vec_to_class_vec(value.class_list)?;
		let intership_list =
			Vec::<Internship>::try_from_internshipdto_vec_to_internship_vec(value.intership_list)?;

		let password = generate_password().map_err(|_| ())?;

		Ok(Self {
			id: Uuid::new_v4().to_string(),
			login: value.login,
			password,
			name: value.name,
			mail: value.mail,
			class_list,
			intership_list,
		})
	}
}

#[async_trait]
impl Db for University {
	async fn insert(&self) -> Result<String, String> {
		let client = Self::setup_database().await?;
		let password_hash = hash_password(&self.password)?;

		client
			.query_one(
				"INSERT INTO university (id, name, mail, login, password) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
				&[&self.id, &self.name, &self.mail, &self.login, &password_hash],
			)
			.await
			.map_err(|e| format!("INSERT error: {e}"))?;

		Ok(format!(
			"Values {}, {}, {}, {password_hash} (encoded password) inserted with id {}",
			self.name, self.mail, self.login, self.id
		))
	}

	async fn login(login: &str, password: &str) -> Result<Self, String>
	where
		Self: Sized,
	{
		let client = Self::setup_database().await?;

		let row = client
			.query_one("SELECT password from university WHERE login=$1", &[&login])
			.await
			.map_err(|e| format!("SELECT error: {e}"))?;

		let hashed_password: String = row.get(0);

		if verify_password(password, &hashed_password)? {
			let row = client
				.query_one(
					"SELECT id, name, login, password, mail from university WHERE login=$1",
					&[&login],
				)
				.await
				.map_err(|e| format!("SELECT error: {e}"))?;

			let id: String = row.get(0);
			let name: String = row.get(1);
			let login: String = row.get(2);
			let password: String = row.get(3);
			let mail: String = row.get(4);

			let university = Self {
				id,
				login,
				password,
				name,
				mail,
				class_list: vec![],     //WIP
				intership_list: vec![], //WIP
			};
			Ok(university)
		} else {
			Err("password incorrect".to_string())
		}
	}
}

impl University {
	pub async fn from_id(id: &str) -> Result<Self, String> {
		let client = Self::setup_database().await?;

		let row = client
			.query_one(
				"SELECT name, login, password, mail FROM university WHERE id=$1;",
				&[&id],
			)
			.await
			.map_err(|e| format!("SELECT error: {e}"))?;

		let name: String = row.get(0);
		let login: String = row.get(1);
		let password: String = row.get(2);
		let mail: String = row.get(3);

		Ok(Self {
			id: id.to_string(),
			login,
			password,
			name,
			mail,
			class_list: Vec::new(),     //WIP
			intership_list: Vec::new(), //WIP
		})
	}
}
