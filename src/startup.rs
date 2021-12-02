use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use crate::routes::{subscribe, health_check};

pub fn run(_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .route("/subscriptions", web::post().to(subscribe))
        .route("/health_check", web::get().to(health_check)))
        .bind("localhost:8000")?
        .run();

    Ok(server)
}