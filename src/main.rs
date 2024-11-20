use crate::routes::platform::platform_routes;
use crate::routes::token::token_routes;
use crate::routes::wechatpay::wechatpay_routes;
use crate::services::platform::DataPlatformService;
use actix_web::{web, App, HttpServer};
use log::info;
use log::warn;
use services::token::TokenService;
use services::wechatpay::WechatPayService;

mod config;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match log4rs::init_file("conf/log.yaml", Default::default()) {
        Ok(_) => info!("[main] log4rs init succ"),
        Err(e) => {
            eprintln!("Failed to initialize log4rs: {}", e);
            std::process::exit(1);
        }
    }

    let integration_conf = web::Data::new(
        config::load_integration_config()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
    );
    let mysql_state = web::Data::new(
        DataPlatformService::new(&integration_conf)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::TimedOut, e))?,
    );
    let wechatpay_state =
        web::Data::new(WechatPayService::new(&integration_conf).map_err(|e| {
            warn!("[main] WechatPayService init failed error: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?);
    let token_state = web::Data::new(TokenService::new());
    HttpServer::new(move || {
        App::new()
            .app_data(integration_conf.clone())
            .app_data(mysql_state.clone())
            .configure(platform_routes)
            .app_data(wechatpay_state.clone())
            .configure(wechatpay_routes)
            .app_data(token_state.clone())
            .configure(token_routes)
    })
    .bind("127.0.0.1:5099")?
    .run()
    .await
}
