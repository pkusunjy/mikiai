use crate::models::Task;
use crate::database::mysql;
use crate::services::AppState;
use actix_web::{web, HttpResponse, Responder};

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .route("", web::post().to(create_task))
            .route("/{id}", web::get().to(get_task))
            .route("/{id}", web::put().to(update_task))
            .route("/{id}", web::delete().to(delete_task))
            .route("/platform/whitelist_query", web::post().to(|| async {
                HttpResponse::Ok().body("app")            })),
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
