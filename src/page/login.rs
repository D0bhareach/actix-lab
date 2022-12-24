use crate::error::ActixLabError;
use actix_web::{
    web, Result, Responder, Error, Scope,
};
use actix_web_lab::respond::Html;

async fn login(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let ctx = tera::Context::new();
    let res = tmpl.render("login.html", &ctx).map_err(|e| {
        log::error!("{}", e);
        ActixLabError::TemplateError(e)
    })?;

    Ok(Html(res))
}

async fn logout(_tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    Ok("Logged out.")
}
pub fn login_scope() -> Scope {
    web::scope("/session")
    .service(web::resource("/login").route(web::get().to(login)))
    .service(web::resource("/logout").route(web::get().to(logout)))
}