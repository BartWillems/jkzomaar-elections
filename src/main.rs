//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use uuid::Uuid;

mod elections;
mod errors;
mod websocket;

#[actix_web::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    init().await?;

    Ok(())
}

pub(crate) struct State {
    election: elections::Election,
    ws: Addr<elections::ElectionServer>,
}

async fn init() -> std::io::Result<()> {
    env_logger::init();

    // let job_server = jobs::JobServer::new().start();
    HttpServer::new(move || {
        let state = State {
            election: elections::Election::new(),
            ws: elections::ElectionServer::new().start(),
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
