use crate::routes::{health_check, subscribe};
use actix_web::middleware::Logger;
use actix_web::HttpServer;
use actix_web::{dev::Server, web, App};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = {
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
                .app_data(pool.clone())
        })
        .listen(listener)?
        .run()
    };
    Ok(server)
}
