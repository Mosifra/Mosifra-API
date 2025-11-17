use rocket::{http::Status, serde::json::Json};

use crate::models::{auth::UserJwt, courses::get::domain::{GetClassesPayload, GetClassesResponse}};

#[post("/courses/classes", data = "<get_classes_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_classes(get_classes_payload: Json<GetClassesPayload>) -> Result<Json<GetClassesResponse>, Status> {
    let raw_jwt = get_classes_payload.into_inner().jwt;

    let user_jwt = UserJwt::from_raw_jwt(&raw_jwt)?;
    match user_jwt {
        Some(user_jwt) => user_jwt.get_classes().await,
        None => Ok(Json(GetClassesResponse {
            success: false,
            classes: None,
        })),
    }
}