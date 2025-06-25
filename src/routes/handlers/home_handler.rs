use actix_web::{Responder, get, web};
use sea_orm::{ConnectionTrait, Statement};

use crate::utils::{api_response::{self, ApiResponse}, app_state::AppState};

#[get("/hello/{name}")]
pub async fn hello_name(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello, {}!", name))
}

#[get("/test")]
pub async fn test(app_state: web::Data<AppState>) -> Result<ApiResponse,ApiResponse> {
    let _res = app_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM user; ",
        ))
        .await
        .map_err(|err| {
            api_response::ApiResponse::new(500, format!("Database query failed: {}", err))
        })?;
    Ok(api_response::ApiResponse::new(200, "This is a test response".to_string()))
}
