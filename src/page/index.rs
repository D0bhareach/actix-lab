use actix_web::{error, web, Error, HttpResponse, Resource, Responder, Result};
use actix_web_lab::respond::Html;
use tera::Tera;

async fn hello(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Index Page");
    ctx.insert("items", &["templates", "middleware", "cookies"]);
    ctx.insert("name", "Actix");
    ctx.insert("text", "Welcome!");
    let res = tmpl.render("index.html", &ctx).map_err(|e| {
        log::error!("{}", e);
        error::ErrorInternalServerError("Template error")
    })?;

    Ok(Html(res))
}

pub fn index_resourse() -> Resource {
    return Resource::new("/").name("index").route(web::get().to(hello));
}
