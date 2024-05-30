use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;

use crate::config::Config;
use crate::routes::routes;

mod config;
mod handlers;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = Config::from_env().expect("Failed to load configuration");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
