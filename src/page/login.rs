use std::collections::HashMap;

use crate::error::ActixLabError;
use actix_web::{http::StatusCode, middleware, web, Error, HttpResponse, Responder, Result, Scope};
use actix_web_lab::respond::Html;
use log;
use serde::Deserialize;

// login_post to get user from somewhere and set session
// in case of an error load page from template and add context.
pub fn default_headers() -> middleware::DefaultHeaders {
    let h = middleware::DefaultHeaders::new();
    h.add(("CacheControl", r#"no-cache="SetCookie, SetCookie2""#))
}

fn render_login_page(
    tmpl: web::Data<tera::Tera>,
    context: Option<tera::Context>,
) -> Result<String, Error> {
    let ctx = if let Some(ctx) = context {
        ctx
    } else {
        tera::Context::new()
    };
    let res = tmpl.render("login.html", &ctx).map_err(|e| {
        log::error!("Error rendering index.html. {}", e);
        ActixLabError::TemplateError(e)
    })?;
    Ok(res)
}

async fn login(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    Ok(Html(render_login_page(tmpl, None)?))
}

async fn logout(_tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    Ok("Logged out.")
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

fn check_user(form: web::Form<LoginForm>) -> bool {
    let mut map = HashMap::with_capacity(2);
    map.insert("admin", "pass");
    map.insert("d0bhareach", "pass");

    let name = form.username.clone();
    let pass = form.password.clone();
    if let Some(mpass) = map.get(name.as_str()) {
        return *mpass == pass;
    }
    false
}

async fn login_post(
    // when success must create Session.
    form: web::Form<LoginForm>,
    t: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    // validate user input if success go to index page.
    if check_user(form) {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish());
    }
    let mut ctx = tera::Context::new();
    ctx.insert("message", &"Error message. Something went wrong.");
    let template = render_login_page(t, Some(ctx));

    Ok(actix_web::HttpResponseBuilder::new(StatusCode::OK)
        .content_type("html")
        .body(template?))
}

pub fn login_scope() -> Scope {
    web::scope("/session")
        // this method is returning Scope but with other generic arguments see above.
        // .wrap(middleware::DefaultHeaders::new())
        .service(
            web::resource("/login")
                .route(web::get().to(login))
                .route(web::post().to(login_post)),
        )
        // .service(web::resource("/login").route(web::post().to(login_post)))
        .service(web::resource("/logout").route(web::get().to(logout)))
}
