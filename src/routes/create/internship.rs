use rocket::{http::Status, serde::json::Json};
use uuid::Uuid;

use crate::models::{auth::AuthGuard, courses::Internship};

use super::domain::{CreateInternshipResponse, CreateIntershipPayload};

#[post("/create/internship", data = "<create_internship_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn create_internship(
	auth: AuthGuard,
	create_internship_payload: Json<CreateIntershipPayload>,
) -> Result<Json<CreateInternshipResponse>, Status> {
	let payload = create_internship_payload.into_inner();
	let user = auth.get_generic_user().await?;

	if user.is_company() {
		let company = user.to_company()?;

		let internship = Internship {
			id: Uuid::new_v4().to_string(),
			course_type: payload.course_type,
			date_start: payload.start_date,
			date_end: payload.end_date,
			min_internship_length: payload.min_internship_length,
			max_internship_length: payload.max_internship_length,
			title: payload.title,
			description: payload.description,
			place: payload.place,
		};

		internship.insert_with_company(company.id.clone()).await?;

		Ok(Json(CreateInternshipResponse { success: true }))
	} else {
		Err(Status::Unauthorized)
	}
}
