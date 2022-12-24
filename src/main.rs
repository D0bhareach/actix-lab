mod page;
mod error;
use actix_files::Files;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder,
    http::StatusCode
};
use page::{index, error as err, login};
use error::handler::not_found_handler;
// TODO:
// create pages modules: index, user, login, error.
// in modules create services and attach middleware for wrapping tera render to responce
// in error mod create handlers for 404 and 500. Add error handlers to root in app.

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info, actix_lab=trace");
    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    env_logger::init();

    HttpServer::new(|| {
        // there is one more instance of tera with exact the same settings in handlers for errors
        let tera =
            tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(middleware::ErrorHandlers::new()
                .handler(StatusCode::NOT_FOUND, not_found_handler),)
            .wrap(middleware::Logger::default())
            .service(Files::new("/public", "static"))
            .service(index::index_scope())
            .service(login::login_scope())
            .service(err::error_scope())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
