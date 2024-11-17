use crate::services::platform::{DataPlatformService, WhitelistUserData};
use actix_web::{web, HttpResponse, Responder};
use log::warn;

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
    client: web::Data<DataPlatformService>,
) -> impl Responder {
    match client.delete_whitelist_user(user.into_inner()).await {
        Ok(rows_affected) => HttpResponse::Ok().body(format!("{{\"res\":{}}}", rows_affected)),
        Err(e) => {
            warn!("[delete_whitelist_user] error: {}", e);
            HttpResponse::Ok().body("{{\"res\":0}}")
        }
    }
}

async fn insert_whitelist_user(
    user: web::Json<WhitelistUserData>,
    client: web::Data<DataPlatformService>,
) -> impl Responder {
    match client.insert_whitelist_user(user.into_inner()).await {
        Ok(rows_affected) => HttpResponse::Ok().body(format!("{{\"res\":{}}}", rows_affected)),
        Err(e) => {
            warn!("[insert_whitelist_user] error: {}", e);
            HttpResponse::Ok().body("{{\"res\":0}}")
        }
    }
}

async fn query_whitelist_user(
    user: web::Json<WhitelistUserData>,
    client: web::Data<DataPlatformService>,
) -> impl Responder {
    match client.query_whitelist_user(user.into_inner()).await {
        Ok(res) => {
            let resp_json_str = match serde_json::to_string(&res) {
                Ok(json_str) => json_str,
                Err(e) => {
                    warn!("[query_whitelist_user] serde to json failed err:{}", e);
                    String::from("[]")
                }
            };
            HttpResponse::Ok().body(resp_json_str)
        }
        Err(e) => {
            warn!("[query_whitelist_user] error: {}", e);
            HttpResponse::Ok().body("[]")
        }
    }
}

async fn update_whitelist_user(
    user: web::Json<WhitelistUserData>,
    client: web::Data<DataPlatformService>,
) -> impl Responder {
    match client.update_whitelist_user(user.into_inner()).await {
        Ok(rows_affected) => HttpResponse::Ok().body(format!("{{\"res\":{}}}", rows_affected)),
        Err(e) => {
            warn!("[update_whitelist_user] error: {}", e);
            HttpResponse::Ok().body("{{\"res\":0}}")
        }
    }
}
