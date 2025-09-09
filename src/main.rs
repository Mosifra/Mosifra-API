use rocket::serde::{Serialize, json::Json};

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Person {
	name: String,
	age: u8,
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> Json<Person> {
	Json(Person {
		name: name.to_string(),
		age,
	})
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![hello])
}
