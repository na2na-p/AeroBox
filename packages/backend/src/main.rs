use actix_web::{App, HttpServer, middleware::Logger, web};
use aws_config::BehaviorVersion;
use dotenv::dotenv;

use crate::config::Config;
use crate::routes::routes;
use crate::services::s3::S3Service;

mod config;
mod handlers;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = Config::from_env().expect("Failed to load configuration");
    let aws_shared_config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(&config.aws_endpoint_url)
        .load()
        .await;
    let s3_service = S3Service::new(config.clone(), aws_shared_config).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(s3_service.clone()))
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
