use crate::{error::ActixLabError};
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{web, Error, Responder, Result, Scope};
use actix_web_lab::respond::Html;
use crate::db;

async fn hello(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<db::Pool>,
    user: Option<Identity>,
    session: Session,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    let username: String = session
        .get("username")
        .unwrap_or_else(|_| Some("Actix".into()))
        .unwrap_or_else(|| "Ananymous".into());

    if let Some(_user) = user {
        // toggle login / logout button in navigation.
        ctx.insert("logged", &true);
    }
    let genres = db::execute(&pool, db::Queries::GetGenres).await?;
    match genres {
        db::DbEntity::Genre(s) => ctx.insert("items", &s),
        // _ => unreachable!()
    }
    ctx.insert("name", &username);
    ctx.insert("title", "Index Page");
    ctx.insert("text", "Welcome!");
    let res = tmpl.render("index.html", &ctx).map_err(|e| {
        tracing::error!("{}", e);
        ActixLabError::TemplateError(e)
    })?;

    Ok(Html(res))
}

pub fn index_scope() -> Scope {
    web::scope("/").service(web::resource("").route(web::get().to(hello)))
}
