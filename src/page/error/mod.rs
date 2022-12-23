// create this module when I was handling errors with redirect
// leave it here for some time to see if I will have some use of it.
// This entire module can be removed if I won't find any use.
use actix_web::{error, web, Error, Responder, Result, Scope};
use actix_web_lab::respond::Html;

async fn not_found_page(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let ctx = tera::Context::new();
    let res = tmpl.render("404.html", &ctx).map_err(|e| {
        log::error!("{}", e);
        error::ErrorInternalServerError("Template error")
    })?;

    Ok(Html(res))
}

pub fn error_scope() -> Scope {
    web::scope("/error").service(web::resource("404").route(web::get().to(not_found_page)))
    // Resource::new("/").name("index").route(web::get().to(hello))
}