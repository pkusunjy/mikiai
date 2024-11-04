use actix_web::{web, HttpResponse, Responder};
use log::{info, warn};

pub fn wechat_pay_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/wx_payment.WxPaymentService").route("/jsapi", web::post().to(jsapi)));
}

async fn jsapi() -> impl Responder {
    info!("jsapi triggered");
    HttpResponse::Ok().body("test")
}
