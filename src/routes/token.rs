use crate::{
    config::IntegrationConfig,
    services::token::{CodeToSessionRequest, TokenService},
};
use actix_web::{web, HttpResponse, Responder};
use log::warn;
use serde_json::json;

pub fn token_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth.AuthService")
            .route("/get_oss_token", web::post().to(get_aliyun_oss_token))
            .route(
                "/get_wx_miniprogram_token",
                web::post().to(get_wechat_miniprogram_token),
            )
            .route("/jscode2session", web::post().to(jscode_to_session)),
    );
}

async fn get_aliyun_oss_token(conf: web::Data<IntegrationConfig>) -> impl Responder {
    let resp_json = json!({
        "ossEndpoint": conf.aliyun_oss_endpoint,
        "ossAccessKeyId": conf.aliyun_oss_access_key_id,
        "ossAccessKeySecret": conf.aliyun_oss_access_key_secret,
    });
    let resp_json_str = match serde_json::to_string(&resp_json) {
        Ok(resp_json_str) => resp_json_str,
        Err(e) => {
            warn!("[get_aliyun_oss_token] serde to json failed err:{}", e);
            String::from("{}")
        }
    };
    HttpResponse::Ok().body(resp_json_str)
}

async fn get_wechat_miniprogram_token(conf: web::Data<IntegrationConfig>) -> impl Responder {
    let resp_json = json!({
        "appid": conf.wechat_appid,
        "secret": conf.wechat_secret,
    });
    let resp_json_str = match serde_json::to_string(&resp_json) {
        Ok(resp_json_str) => resp_json_str,
        Err(e) => {
            warn!(
                "[get_wechat_miniprogram_token] serde to json failed err:{}",
                e
            );
            String::from("{}")
        }
    };
    HttpResponse::Ok().body(resp_json_str)
}

async fn jscode_to_session(
    req: web::Json<CodeToSessionRequest>,
    conf: web::Data<IntegrationConfig>,
    token_service: web::Data<TokenService>,
) -> impl Responder {
    match token_service
        .jscode_to_session(&conf, req.into_inner())
        .await
    {
        Ok(resp) => {
            let resp_json_str = match serde_json::to_string(&resp) {
                Ok(json_str) => json_str,
                Err(e) => {
                    warn!("[jscode_to_session] serde to json failed err:{}", e);
                    String::from("{}")
                }
            };
            HttpResponse::Ok().body(resp_json_str)
        }
        Err(e) => {
            warn!("[jscode_to_session] error: {}", e);
            HttpResponse::Ok().body("{}")
        }
    }
}
