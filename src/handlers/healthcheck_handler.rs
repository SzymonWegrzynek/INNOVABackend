use actix_web::HttpResponse;

pub struct HealthCheck;

impl HealthCheck {
    pub async fn healthcheck() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
