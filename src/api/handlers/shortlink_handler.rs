use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde_json::json;

use crate::api::prelude::*;

pub async fn create_shortlink(
    payload: web::Json<CreateShortLinkDto>,
    pool: web::Data<DbPool>,
    session: Session,
    req: HttpRequest,
) -> HttpResponse {
    let mut conn = match get_connection(&pool) {
        Ok(c) => c,
        Err(err) => return err.error_response(),
    };

    let user_id = is_authenticated(&session, &mut conn).await.ok();
    let anon_id = is_anonymous(&session).await;

    match generate_shortlink(user_id, anon_id, payload.into_inner(), &mut conn).await {
        Ok(result) => {
            let url = req
                .url_for("redirect", &[result.hash.unwrap()])
                .ok()
                .unwrap()
                .to_string();
            HttpResponse::Ok().json(json!({
                "data": {
                    "short_url": url
                }
            }))
        }
        Err(err) => err.error_response(),
    }
}

pub async fn get_shortlinks(
    pool: web::Data<DbPool>,
    session: Session,
    req: HttpRequest,
) -> HttpResponse {
    let mut conn = match get_connection(&pool) {
        Ok(c) => c,
        Err(err) => return err.error_response(),
    };

    let user_id = match is_authenticated(&session, &mut conn).await {
        Ok(id) => id,
        Err(err) => return err.error_response(),
    };

    match list_user_shortlinks(user_id, &mut conn).await {
        Ok(data) => {
            let result = data
                .iter()
                .map(|shortlink| {
                    let hash = shortlink.hash.as_ref().unwrap();
                    let url = req.url_for("redirect", &[hash]).ok().map(|v| v.to_string());
                    ShortLinkDto {
                        long_url: shortlink.long_url.to_owned(),
                        short_url: url,
                        status: shortlink.status.to_owned(),
                        created_at: shortlink.created_at,
                        is_private: shortlink.is_private,
                    }
                })
                .collect::<Vec<ShortLinkDto>>();

            HttpResponse::Ok().json(json!({
                "data": result
            }))
        }
        Err(err) => err.error_response(),
    }
}

pub async fn redirect(path: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let code = path.into_inner();
    let mut conn = match get_connection(&pool) {
        Ok(c) => c,
        Err(err) => return err.error_response(),
    };

    match get_long_url(&code, &mut conn).await {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
