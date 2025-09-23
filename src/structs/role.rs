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
			Self::Admin => write!(f, "Admin"),
			Self::University => write!(f, "University"),
			Self::Student => write!(f, "Student"),
			Self::Alumni => write!(f, "Alumni"),
			Self::Company => write!(f, "Company"),
		}
	}
}
