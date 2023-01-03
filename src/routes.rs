use crate::IftttKey;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};

#[tracing::instrument(name = "Health check")]
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    tracing::info!("Health check bounce");
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SendMessagePath {
    command: String,
}

#[derive(serde::Deserialize)]
struct IftttQueryKey {
    key: String,
}

#[post("/ifttt/{command}")]
async fn send_message(
    path: web::Path<SendMessagePath>,
    query: web::Query<IftttQueryKey>,
    // mqtt_service: web::Data<()>,
    ifttt_key: web::Data<IftttKey>,
) -> impl Responder {
    if !ifttt_key.0.eq(&query.key) {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}
