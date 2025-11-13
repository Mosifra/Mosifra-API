use rocket::http::Status;

pub trait StatusResultHandling<T, E: std::fmt::Debug> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status>;
	fn internal_server_error_no_message(self) -> Result<T, Status>;
}

impl<T, E: std::fmt::Debug> StatusResultHandling<T, E> for Result<T, E> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status> {
		match self {
			Ok(value) => Ok(value),
			Err(e) => {
				eprintln!("{} : {e:?}", message.to_string());
				Err(Status::InternalServerError)
			}
		}
	}

	fn internal_server_error_no_message(self) -> Result<T, Status> {
		match self {
			Ok(value) => Ok(value),
			Err(e) => {
				eprintln!("{e:?}");
				Err(Status::InternalServerError)
			}
		}
	}
}

pub trait StatusOptionHandling<T> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status>;
}

impl<T> StatusOptionHandling<T> for Option<T> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status> {
		match self {
			Some(value) => Ok(value),
			None => {
				eprintln!("{}", message.to_string());
				Err(Status::InternalServerError)
			}
		}
	}
}
