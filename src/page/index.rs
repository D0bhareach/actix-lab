use crate::error::ActixLabError;
use actix_web::{web, Error, Responder, Result, Scope};
use actix_web_lab::respond::Html;

async fn hello(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Index Page");
    ctx.insert("items", &["templates", "middleware", "cookies"]);
    ctx.insert("name", "Actix");
    ctx.insert("text", "Welcome!");
    let res = tmpl.render("index.html", &ctx).map_err(|e| {
        log::error!("{}", e);
        ActixLabError::TemplateError(e)
    })?;

    Ok(Html(res))
}

pub fn index_scope() -> Scope {
    web::scope("/").service(web::resource("").route(web::get().to(hello)))
}
