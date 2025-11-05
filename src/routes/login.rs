use rocket::{
    form::Form,
    http::{Cookie, CookieJar, SameSite},
};
use uuid::Uuid;

use crate::{
    redis::{self, SessionData, get_user_id_from_twofa},
    structs::{company::Company, student::Student, university::University},
    traits::db::Db,
    utils::set_transaction_id,
};

#[derive(Debug, FromForm)]
pub struct LoginForm {
    login: String,
    password: String,
    remember_me: bool,
}

#[derive(Debug, FromForm)]
pub struct Twofa {
    pub code: String,
    pub transaction_id: String,
    pub user_type: String,
    pub remember_me: bool,
}

#[post("/login_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_university(form: Form<LoginForm>) -> Result<String, String> {
    let login = form.into_inner();
    let university = University::login(&login.login, &login.password).await;

    match university {
        Ok(university) => {
            set_transaction_id(&university.mail, &university.id, login.remember_me).await
        }
        Err(e) => Err(format!("Invalid Password: {e}")),
    }
}

#[post("/login_company", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_company(form: Form<LoginForm>) -> Result<String, String> {
    let login = form.into_inner();
    let company = Company::login(&login.login, &login.password).await;

    match company {
        Ok(company) => set_transaction_id(&company.mail, &company.id, login.remember_me).await,
        Err(e) => Err(format!("Invalid Password: {e}")),
    }
}

#[post("/login_student", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_student(form: Form<LoginForm>) -> Result<String, String> {
    let login = form.into_inner();
    let student = Student::login(&login.login, &login.password).await;

    match student {
        Ok(student) => set_transaction_id(&student.mail, &student.id, login.remember_me).await,
        Err(e) => Err(format!("Invalid Password: {e}")),
    }
}

#[post("/twofa", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub fn twofa(form: Form<Twofa>, cookies: &CookieJar<'_>) -> Result<String, String> {
    let twofa = form.into_inner();

    if redis::check_2fa_code(&twofa)? {
        let session_id = Uuid::new_v4().to_string();
        let session_data = SessionData {
            user_id: get_user_id_from_twofa(&twofa)?.to_string(),
            user_type: twofa.user_type.clone(),
        };

        let ttl_seconds: u64 = if twofa.remember_me {
            30 * 24 * 3600
        } else {
            30 * 60
        };
        redis::set_session(&session_id, &session_data, ttl_seconds)?;
        redis::invalidate_transactionid(&twofa)?;

        cookies.add(
            Cookie::build(("session_id", session_id))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .build(),
        );

        cookies.add(
            Cookie::build(("userType", twofa.user_type))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .build(),
        );

        Ok("Logged in".to_string())
    } else {
        Ok("Incorrect code".to_string())
    }
}
