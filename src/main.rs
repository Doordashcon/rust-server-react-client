#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod attendance_handler;
mod errors;
mod invitation_handler;
mod models;
mod register_handler;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to crate pool");

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .data(web::JsonConfig::default().limit(4096))
            // everything under "/api" route
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/invitation")
                            .route(web::post().to(invitation_handler::post_invitation))
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to(register_handler::register_user))
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(attendance_handler::fomo))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
