mod page;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use page::index;
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
        App::new()
            .wrap(middleware::Logger::default())
            .service(index::index_resourse())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
