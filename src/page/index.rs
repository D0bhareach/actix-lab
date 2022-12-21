use actix_web::{web, HttpResponse, Resource, Responder};

async fn hello() -> impl Responder {
    log::info!("INFO Message for index handler.");
    log::error!("Hello error!!");
    log::debug!("Hello debug!!");
    log::warn!("Hello warn!!");
    HttpResponse::Ok().body("Hello world!")
}

pub fn index_resourse() -> Resource {
    return Resource::new("/").name("index").route(web::get().to(hello));
}
