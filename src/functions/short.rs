use leptos::prelude::*;

cfg_if::cfg_if! {

    if #[cfg(feature="ssr")] {
        use actix_web::{
            web::Data,
            HttpRequest
        };
        use actix_session::Session;
        use leptos_actix::{extract};
        use crate::{
            api::{
                dto::{CreateShortLinkDto},
                prelude::{get_connection, is_authenticated, generate_shortlink, is_anonymous},
            },
            prelude::DbPool,
        };
    }

}

#[server]
pub async fn create_shortlink(url: String) -> Result<String, ServerFnError> {
    let pool = extract::<Data<DbPool>>()
        .await
        .map_err(|_| ServerFnError::new("Database pool not found"))?;
    let mut conn = get_connection(&pool).map_err(|err| ServerFnError::new(err.to_string()))?;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|_| ServerFnError::new("Http Request is not found"))?;

    let session = extract::<Session>()
        .await
        .map_err(|_| ServerFnError::new("Session not found"))?;

    let user_id = is_authenticated(&session, &mut conn).await.ok();
    let mut anon_id = is_anonymous(&session).await;

    if user_id.is_none() && anon_id.is_none() {
        let id = uuid::Uuid::new_v4();
        anon_id = Some(id);
        let _ = session.insert("is_guest", id);
    }

    match generate_shortlink(user_id, anon_id, CreateShortLinkDto { url }, &mut conn).await {
        Ok(result) => {
            let code = result.hash.unwrap();
            let url = req
                .url_for("redirect", &[code])
                .map_err(|e| ServerFnError::new(format!("Failed to generate URL: {}", e)))?
                .to_string();
            return Ok(url);
        }
        Err(_) => Err(ServerFnError::new("Something went wrong! try again later.")),
    }
}
