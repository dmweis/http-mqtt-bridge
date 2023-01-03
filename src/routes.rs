use crate::IftttKey;
use actix_web::{get, post, web, HttpResponse, Responder};
use rumqttc::AsyncClient;

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

#[post("/ifttt_simple/{command}")]
async fn send_message(
    path: web::Path<SendMessagePath>,
    query: web::Query<IftttQueryKey>,
    mqtt_service: web::Data<AsyncClient>,
    ifttt_key: web::Data<IftttKey>,
) -> impl Responder {
    if !ifttt_key.0.eq(&query.key) {
        tracing::warn!("unauthenticated user");
        HttpResponse::NotFound().finish()
    } else {
        tracing::info!("Sending message");
        mqtt_service
            .publish(
                "ifttt_simple",
                rumqttc::QoS::AtMostOnce,
                false,
                path.command.as_bytes(),
            )
            .await
            .unwrap();
        HttpResponse::Ok().finish()
    }
}
