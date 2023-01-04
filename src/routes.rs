use crate::IftttKey;
use actix_web::{get, post, web, HttpResponse, Responder};
use rumqttc::AsyncClient;

#[tracing::instrument(name = "Health check")]
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SendMessagePath {
    topic: String,
}

#[derive(serde::Deserialize)]
struct IftttQueryKey {
    key: String,
}

#[post("/ifttt_simple/{topic}")]
async fn send_message(
    path: web::Path<SendMessagePath>,
    query: web::Query<IftttQueryKey>,
    mqtt_service: web::Data<AsyncClient>,
    ifttt_key: web::Data<IftttKey>,
    body: String,
) -> impl Responder {
    if !ifttt_key.0.eq(&query.key) {
        tracing::warn!("unauthenticated user");
        HttpResponse::NotFound().finish()
    } else {
        tracing::info!("Sending message");
        mqtt_service
            .publish(
                &path.topic,
                rumqttc::QoS::AtMostOnce,
                false,
                body.as_bytes(),
            )
            .await
            .unwrap();
        HttpResponse::Ok().finish()
    }
}
