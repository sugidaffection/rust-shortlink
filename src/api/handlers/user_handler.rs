use actix_session::Session;
use actix_web::{web, HttpResponse, ResponseError};
use serde_json::json;

use crate::api::prelude::*;

pub async fn profile(pool: web::Data<DbPool>, session: Session) -> HttpResponse {
    let user_id = match session.get::<uuid::Uuid>("session") {
        Ok(Some(id)) => id,
        Ok(None) => return ApiError::Unauthorized.error_response(),
        Err(_) => return ApiError::InternalError.error_response(),
    };

    let mut conn = match get_connection(&pool) {
        Ok(c) => c,
        Err(err) => return err.error_response(),
    };

    match get_user_by_id(user_id, &mut conn).await {
        Ok(user) => HttpResponse::Ok().json(json!({ "data": user })),
        Err(_) => ApiError::Unauthorized.error_response(),
    }
}
