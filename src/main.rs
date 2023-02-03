mod error;
mod page;
mod db;
use db::Pool;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{
    config::{PersistentSession, TtlExtensionPolicy},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, Key},
    http::StatusCode,
    middleware, web, App, HttpServer,
};
use anyhow::Context;
use error::handler::{internal_error_handler, not_found_handler};
use std::collections::HashMap;
use page::{error as err, index, login};
use r2d2_sqlite::{self, SqliteConnectionManager};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};
use tracing_actix_web::TracingLogger;
use tracing_log::LogTracer;

fn get_env_var(config_map: &HashMap<String, String>, key: &str) -> Result<String, anyhow::Error> {
    let key = config_map
        .get(key)
        .context(format!("can not get key: {} from the .env file", key))?;
    Ok(key.to_string())
}


fn load_rustls_config(cert_path: &str, key_path: &str) -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(key_path).unwrap());

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Unable to setup log tracer!");
    let configs_map = dotenvy::vars().collect::<HashMap<String, String>>();

    let cert_path = get_env_var(&configs_map, "tls_cert_file").unwrap();
    let key_path = get_env_var(&configs_map, "tls_key_file").unwrap();
    let cookie_key = get_env_var(&configs_map, "cookie_key").unwrap();
    let session_ttl: i64 = get_env_var(&configs_map, "ttl").unwrap().parse().unwrap();
    let db_file = get_env_var(&configs_map, "db_file").unwrap();

    // instances
    let tera =
        tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();
    let tls_config = load_rustls_config(&cert_path, &key_path);
    // connect to SQLite DB
    let manager = SqliteConnectionManager::file(&db_file);
    let pool = Pool::new(manager).unwrap();


    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());

    // TODO: not ready for production!

    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking_writer)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    HttpServer::new(move || {
        // there is one more instance of tera with exact the same settings in handlers for errors
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(cookie_key.clone().as_bytes()),
                )
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(Duration::seconds(session_ttl))
                        .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                )
                .cookie_secure(true)
                .build(),
            )
            .wrap(
                middleware::ErrorHandlers::new()
                    .handler(StatusCode::NOT_FOUND, not_found_handler)
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error_handler),
            )
            // .wrap(middleware::Logger::default())
            .service(Files::new("/public", "static"))
            .service(index::index_scope())
            .service(login::login_scope().wrap(login::default_headers()))
            .service(err::error_scope())
    })
    .workers(2)
    .bind_rustls("127.0.0.1:8443", tls_config)?
    .run()
    .await
}
