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
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(app_state.clone())
            .configure(task_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
