use crate::database::mysql::{MysqlClient, WhitelistUserData};
use actix_web::{web, HttpResponse, Responder};
use log::{info, warn};

pub fn platform_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/platform")
            .route("/whitelist_delete", web::post().to(delete_whitelist_user))
            .route("/whitelist_insert", web::post().to(insert_whitelist_user))
            .route("/whitelist_query", web::post().to(query_whitelist_user))
            .route("/whitelist_update", web::post().to(update_whitelist_user)),
    );
}

async fn delete_whitelist_user(
    user: web::Json<WhitelistUserData>,
    client: web::Data<MysqlClient>,
) -> impl Responder {
    match client.delete_whitelist_user(user.into_inner()).await {
        Ok(rows_affected) => HttpResponse::Ok().body(format!("{{\"res\":{}}}", rows_affected)),
        Err(_) => {
            warn!("delete whitelist user err");
            HttpResponse::Ok().body("error")
        }
    }
}

async fn insert_whitelist_user(
    user: web::Json<WhitelistUserData>,
    client: web::Data<MysqlClient>,
) -> impl Responder {
    match client.insert_whitelist_user(user.into_inner()).await {
        Ok(rows_affected) => HttpResponse::Ok().body(format!("{{\"res\":{}}}", rows_affected)),
        Err(_) => {
            warn!("insert whitelist user err");
            HttpResponse::Ok().body("error")
        }
    }
}

async fn query_whitelist_user(
    user: web::Json<WhitelistUserData>,
    client: web::Data<MysqlClient>,
) -> impl Responder {
    info!("query_whitelist_user triggered received user:{:?}", user);
    match client.query_whitelist_user(user.into_inner()).await {
        Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(e) => HttpResponse::Ok().body(format!("some error:{}", e)),
    }
}

async fn update_whitelist_user(
    user: web::Json<WhitelistUserData>,
    client: web::Data<MysqlClient>,
) -> impl Responder {
    info!("update_whitelist_user triggered received user:{:?}", user);
    match client.update_whitelist_user(user.into_inner()).await {
        Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(e) => HttpResponse::Ok().body(format!("some error:{}", e)),
    }
}
