use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::info;
use sqlx::query;

pub fn forward_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/utility-project")
            .route("/{level_1}/{level_2}", web::post().to(forward))
            .route("/{level_1}/{level_2}", web::get().to(forward)),
    );
}

pub async fn forward(
    level_1: web::Path<String>,
    level_2: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    info!("/utility-project/{}/{} triggered", level_1, level_2);
    let url = req.uri();
    let forward_url = format!("http://{}{}", "endpoint", url.path());
    // for GET method, there should be query
    // for POST method, the query() should return None and skip
    let full_url = match url.query() {
        Some(query) => format!("{}?{}", forward_url, query),
        None => forward_url,
    };
    info!(
        "level_1:{}, level_2:{}, forward generate url: {}",
        level_1, level_2, full_url
    );

    HttpResponse::Ok().body("ok")
}
