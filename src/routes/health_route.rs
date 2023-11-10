use actix_web::{get, HttpResponse, Result};
use serde_json::json;

#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> Result<HttpResponse> {
    const MESSAGE: &str = "Build Simple Crud";
    Ok(HttpResponse::Ok().json(json!({"status": "Success", "message": MESSAGE})))
}
