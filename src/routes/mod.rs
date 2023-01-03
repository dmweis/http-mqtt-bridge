use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};

#[tracing::instrument(name = "Health check")]
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    tracing::info!("Health check bounce");
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SendCommand {
    content: Option<String>,
}

#[post("/send_message")]
async fn send_message(
    command: web::Json<SendCommand>,
    speech_service: web::Data<()>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}
