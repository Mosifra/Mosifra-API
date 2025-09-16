use std::str::FromStr;

use rocket::time::Date;
use uuid::Uuid;

use crate::course_type::CourseType;

// For now
#[allow(dead_code)]
#[derive(Debug)]
pub struct Class {
	id: String,
	name: String,
	course_type: CourseType,
	date_internship_start: Date,
	date_internship_end: Date,
}

#[derive(Debug, FromForm)]
pub struct ClassDto {
	name: String,
	course_type: String,
	date_internship_start: Date,
	date_internship_end: Date,
}

impl TryFrom<ClassDto> for Class {
	type Error = ();

	fn try_from(value: ClassDto) -> Result<Self, Self::Error> {
		Ok(Self {
			id: Uuid::new_v4().to_string(),
			name: value.name,
			course_type: CourseType::from_str(&value.course_type)?,
			date_internship_start: value.date_internship_start,
			date_internship_end: value.date_internship_end,
		})
	}
}

pub trait TryFromVecClassDtoToClassVec {
	// Not sure how to fix this one
	#[allow(clippy::result_unit_err)]
	fn try_from_classdto_vec_to_class_vec(value: Vec<ClassDto>) -> Result<Self, ()>
	where
		Self: std::marker::Sized;
}

impl TryFromVecClassDtoToClassVec for Vec<Class> {
	fn try_from_classdto_vec_to_class_vec(value: Vec<ClassDto>) -> Result<Self, ()> {
		let mut res = vec![];
		for class_dto in value {
			res.push(Class::try_from(class_dto)?);
		}
		Ok(res)
	}
}
