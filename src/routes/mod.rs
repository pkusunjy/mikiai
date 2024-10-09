use crate::database::mysql::{MysqlClient, WhitelistUserData};
use crate::models::Task;
use crate::services::AppState;
use actix_web::{web, HttpResponse, Responder};
use log::{info, warn};

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/task")
            .route("", web::post().to(create_task))
            .route("/{id}", web::get().to(get_task))
            .route("/{id}", web::put().to(update_task))
            .route("/{id}", web::delete().to(delete_task)),
    );
}

pub fn platform_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/platform").route("/whitelist_query", web::post().to(query_whitelist_user)),
    );
}

async fn create_task(task: web::Json<Task>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    tasks.insert(task.id, task.into_inner());
    HttpResponse::Created().finish()
}

async fn get_task(id: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    if let Some(task) = tasks.get(&id) {
        HttpResponse::Ok().json(task)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_task(
    id: web::Path<usize>,
    task: web::Json<Task>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if tasks.contains_key(&id) {
        tasks.insert(*id, task.into_inner());
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_task(id: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if tasks.remove(&id).is_some() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
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
