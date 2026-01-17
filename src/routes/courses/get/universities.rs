use rocket::{http::Status, serde::json::Json};

use crate::{
    models::{auth::AuthGuard, users::GenericUser},
    routes::courses::get::domain::GetUniversitiesResponse,
};

#[get("/courses/universities")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn get_universities(auth: AuthGuard) -> Result<Json<GetUniversitiesResponse>, Status> {
    let generic_user = auth.get_generic_user().await?;
    if generic_user.is_admin() {
        let admin = generic_user.to_admin();
        todo!()
    }
}
