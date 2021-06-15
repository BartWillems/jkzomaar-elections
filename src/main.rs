//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use actix::prelude::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

mod elections;
mod errors;
mod websocket;

#[actix_web::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    init().await?;

    Ok(())
}

pub(crate) struct State {
    ws: Addr<elections::ElectionServer>,
    db: PgPool,
}

async fn init() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db = PgPool::connect("postgres://jkzomaar:secret@127.0.0.1/jkzomaar")
        .await
        .expect("Unable to connect to database");

    let election_server = elections::ElectionServer::new().start();

    HttpServer::new(move || {
        let state = State {
            ws: election_server.clone(),
            db: db.clone(),
        };

        App::new()
            .data(state)
            .wrap(Logger::default())
            .wrap(Cors::permissive().supports_credentials())
            .service(web::scope("/api").configure(elections::routes))
            .service(web::resource("/ws").to(websocket::route))
            .service(mount_frontend())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(target_os = "freebsd")]
fn mount_frontend() -> Files {
    Files::new("/", "frontend").index_file("index.html")
}

#[cfg(not(target_os = "freebsd"))]
fn mount_frontend() -> Files {
    Files::new("/", "frontend/build").index_file("index.html")
}
