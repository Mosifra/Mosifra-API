use std::fmt::Display;

#[derive(Debug)]
pub enum Role {
	Admin,
	University,
	Student,
	Alumni,
	Company,
}

impl Display for Role {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Role::Admin => write!(f, "Admin"),
			Role::University => write!(f, "University"),
			Role::Student => write!(f, "Student"),
			Role::Alumni => write!(f, "Alumni"),
			Role::Company => write!(f, "Company"),
		}
	}
}
