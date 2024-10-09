use crate::database::mysql::MysqlClient;
use crate::routes::platform_routes;
use crate::routes::task_routes;
use actix_web::{web, App, HttpServer};

mod config;
mod database;
mod middleware;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    middleware::logger::setup_logging();

    let app_state = web::Data::new(services::AppState::new());
    let mysql_state = web::Data::new(
        MysqlClient::new()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::TimedOut, e))?,
    );
    HttpServer::new(move || {
        App::new()
            .app_data(mysql_state.clone())
            .configure(platform_routes)
            .app_data(app_state.clone())
            .configure(task_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
