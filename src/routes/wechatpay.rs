use crate::services::{
    platform::DataPlatformService,
    wechatpay::{CheckAndPayRequest, WechatPayService},
};
use actix_web::{web, HttpResponse, Responder};
use log::warn;

pub fn wechatpay_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/wx_payment.WxPaymentService").route("/jsapi", web::post().to(jsapi)));
}

async fn jsapi(
    req: web::Json<CheckAndPayRequest>,
    wechatpay_srv: web::Data<WechatPayService>,
    platform_srv: web::Data<DataPlatformService>,
) -> impl Responder {
    match wechatpay_srv.jsapi(req.into_inner(), &platform_srv).await {
        Ok(jsapi_resp) => {
            let resp_json_str = match serde_json::to_string(&jsapi_resp) {
                Ok(json_str) => json_str,
                Err(e) => {
                    warn!("[jsapi] serde to json failed err:{}", e);
                    String::from("{}")
                }
            };
            HttpResponse::Ok().body(resp_json_str)
        }
        Err(e) => {
            warn!("[jsapi] error: {}", e);
            HttpResponse::Ok().body("{}")
        }
    }
}
