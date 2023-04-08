use crate::routes::{health_check, subscribe};
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // web::get() is a shortcut for Route::new().guard(guard::Get())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
