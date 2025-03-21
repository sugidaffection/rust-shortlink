use actix_session::Session;
use actix_web::{web, HttpResponse, ResponseError};
use serde_json::json;

use crate::api::prelude::*;

pub async fn login(
    payload: web::Json<LoginDto>,
    pool: web::Data<DbPool>,
    session: Session,
) -> HttpResponse {
    let mut conn = match get_connection(&pool) {
        Ok(c) => c,
        Err(err) => return err.error_response(),
    };

    match sign_in(payload.into_inner(), &mut conn).await {
        Ok(id) => {
            if let Err(_) = session.insert("session", id) {
                return ApiError::InternalError.error_response();
            }
            HttpResponse::Ok().json(json!({
                "result": "success"
            }))
        }
        Err(e) => return e.error_response(),
    }
}

pub async fn register(
    payload: web::Json<RegisterDto>,
    pool: web::Data<DbPool>,
    session: Session,
) -> HttpResponse {
    let mut conn = match get_connection(&pool) {
        Ok(c) => c,
        Err(err) => {
            return err.error_response();
        }
    };

    match sign_up(payload.into_inner(), &mut conn).await {
        Ok(id) => {
            if let Err(_) = session.insert("session", id) {
                return ApiError::InternalError.error_response();
            }
            HttpResponse::Ok().json(json!({
                "result": "success"
            }))
        }
        Err(e) => return e.error_response(),
    }
}
