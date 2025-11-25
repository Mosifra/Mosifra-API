use std::any::Any;

use rocket::{http::Status, serde::json::Json};

use crate::{error_handling::StatusOptionHandling, redis, routes::auth::DisconnectResponse};

use super::{Student, University};

pub struct GenericUser {
	session_id: String,
	inner: Box<dyn Any + Send>,
}

impl GenericUser {
	pub fn new<T: 'static + Send>(value: T, session_id: String) -> Self {
		Self {
			inner: Box::new(value),
			session_id,
		}
	}

	pub fn is_university(&self) -> bool {
		self.inner.is::<University>()
	}

	pub fn to_university(&self) -> Result<&University, Status> {
		self.inner
			.downcast_ref::<University>()
			.internal_server_error("Cannot convert GenericUser to University")
	}

	pub fn is_student(&self) -> bool {
		self.inner.is::<Student>()
	}

	pub fn to_student(&self) -> Result<&Student, Status> {
		self.inner
			.downcast_ref::<Student>()
			.internal_server_error("Cannot convert GenericUser to Student")
	}

	pub fn logout(&self) -> Result<Json<DisconnectResponse>, Status> {
		redis::invalidate_session(&self.session_id)?;
		Ok(Json(DisconnectResponse { success: true }))
	}
}
