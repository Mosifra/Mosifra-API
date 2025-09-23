use rocket::form::Form;

use crate::{
    db::{
        get_company_password_from_mail, get_student_password_from_mail,
        get_university_password_from_mail,
        insert_2fa_for_company,
        insert_2fa_for_university,
        insert_2fa_for_student
    },
    utils::{send_2fa_mail, verify_mail, verify_password},
};

#[derive(Debug, FromForm)]
pub struct Login {
    mail: String,
    password: String,
}

#[post("/login_university", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_university(form: Form<Login>) -> Result<String, String> {
    let login = form.into_inner();

    if !verify_mail(&login.mail) {
        return Err("Incorrect Mail".to_string());
    }

    let correct_password = get_university_password_from_mail(&login.mail).await?;

    if verify_password(&login.password, &correct_password)? {
        let code = send_2fa_mail(&login.mail).await?;
        Ok("Sent mail for 2fa".to_string())
    } else {
        Err("Invalid Password".to_string())
    }
}

#[post("/login_company", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_company(form: Form<Login>) -> Result<String, String> {
    let login = form.into_inner();

    if !verify_mail(&login.mail) {
        return Err("Incorrect Mail".to_string());
    }

    let correct_password = get_company_password_from_mail(&login.mail).await?;

    if verify_password(&login.password, &correct_password)? {
        let code = send_2fa_mail(&login.mail).await?;
        Ok("Logged in".to_string())
    } else {
        Err("Invalid Password".to_string())
    }
}

#[post("/login_student", data = "<form>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
pub async fn login_student(form: Form<Login>) -> Result<String, String> {
    let login = form.into_inner();

    if !verify_mail(&login.mail) {
        return Err("Incorrect Mail".to_string());
    }

    let correct_password = get_student_password_from_mail(&login.mail).await?;

    if verify_password(&login.password, &correct_password)? {
        let code = send_2fa_mail(&login.mail).await?;
        //insert_2fa_for_student(id, twofa)     Victor enculé fix l'id
        Ok("Logged in".to_string())
    } else {
        Err("Invalid Password".to_string())
    }
}
