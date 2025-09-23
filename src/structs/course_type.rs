use std::str::FromStr;

#[derive(Debug)]
pub enum CourseType {
	Info,
}

impl FromStr for CourseType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Info" => Ok(Self::Info),
			_ => Err(()),
		}
	}
}
