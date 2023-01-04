mod error;
mod page;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{
    config::{TtlExtensionPolicy,PersistentSession},
    storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key}, http::StatusCode, middleware, web, App, HttpServer,
};
use error::handler::{internal_error_handler, not_found_handler};
// Errors have different behaviour.
use page::{error as err, index, login};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};

// TODO: add redis for holding session data.

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("certificate/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("certificate/key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

// TODO: how to test if browser is caching, how to test no-cache??
// TODO: First test for span / domain shall be headers and cookies tests.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info, actix_lab=trace");
    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    env_logger::init();
    let tls_config = load_rustls_config();
    const COOKIE_KEY: &str = "BX+1s/Og8J7tiPoIBCNvuTIsCL4ehZZRsCt0f9AVvd/dIPGKu4Zu63/OWO87l5M3
ldnMsWhJRWvgZfdMZ6ZvYQ==";
    const SESSION_TTL: i64 = 60 * 3;

    HttpServer::new(|| {
        // there is one more instance of tera with exact the same settings in handlers for errors
        let tera =
            tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(COOKIE_KEY.as_bytes()),
                )
                .session_lifecycle(
                    PersistentSession::default()
                    .session_ttl(Duration::seconds(SESSION_TTL))
                    .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest)
                )
                .cookie_secure(true)
                .build(),
            )
            .wrap(
                middleware::ErrorHandlers::new()
                    .handler(StatusCode::NOT_FOUND, not_found_handler)
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error_handler),
            )
            .wrap(middleware::Logger::default())
            .service(Files::new("/public", "static"))
            .service(index::index_scope())
            .service(login::login_scope().wrap(login::default_headers()))
            .service(err::error_scope())
    })
    .bind_rustls("127.0.0.1:8443", tls_config)?
    .run()
    .await
}
