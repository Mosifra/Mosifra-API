use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::courses::{Class, CourseType};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ClassDto {
	pub id: String,
	pub name: String,
	pub course_type: CourseType,
	pub date_internship_start: NaiveDate,
	pub date_internship_end: NaiveDate,
	pub maximum_internship_length: i32,
	pub minimum_internship_length: i32,
}

impl ClassDto {
	pub fn from_vec(class_list: Vec<Class>) -> Vec<Self> {
		let mut res = vec![];
		for class in class_list {
			res.push(Self {
				id: class.id,
				name: class.name,
				course_type: class.course_type,
				date_internship_start: class.date_internship_start,
				date_internship_end: class.date_internship_end,
				maximum_internship_length: class.maximum_internship_length,
				minimum_internship_length: class.minimum_internship_length,
			});
		}

		res
	}
}
