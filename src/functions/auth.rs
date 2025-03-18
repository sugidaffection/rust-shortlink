use leptos::prelude::*;

cfg_if::cfg_if! {

    if #[cfg(feature="ssr")] {
        use actix_web::{
            web::Data,
        };
        use actix_session::Session;
        use leptos_actix::{extract};
        use secrecy::SecretString;
        use crate::{
            api::{
                dto::{LoginDto, RegisterDto},
                prelude::{get_connection, sign_in, sign_up, is_authenticated},
            },
            prelude::DbPool,
        };
    }

}

#[server]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    let pool = extract::<Data<DbPool>>()
        .await
        .map_err(|_| ServerFnError::new("Database pool not found"))?;
    let mut conn = get_connection(&pool).map_err(|err| ServerFnError::new(err.to_string()))?;

    match sign_in(
        LoginDto {
            email,
            password: SecretString::from(password),
        },
        &mut conn,
    )
    .await
    {
        Ok(id) => {
            let session = match extract::<Session>()
                .await
                .map_err(|_| ServerFnError::new("Session not found"))
            {
                Ok(session) => session,
                Err(err) => return Err(err),
            };

            if let Err(_) = session.insert("session", id) {
                return Err(ServerFnError::new("Something went wrong."));
            }

            leptos_actix::redirect("/");

            Ok(())
        }
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

#[server]
pub async fn register(
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    let pool = extract::<Data<DbPool>>()
        .await
        .map_err(|_| ServerFnError::new("Database pool not found"))?;
    let mut conn = get_connection(&pool).map_err(|err| ServerFnError::new(err.to_string()))?;

    match sign_up(
        RegisterDto {
            username,
            email,
            password: SecretString::from(password),
        },
        &mut conn,
    )
    .await
    {
        Ok(id) => {
            let session = match extract::<Session>()
                .await
                .map_err(|_| ServerFnError::new("Session not found"))
            {
                Ok(session) => session,
                Err(err) => return Err(err),
            };

            if let Err(_) = session.insert("session", id) {
                return Err(ServerFnError::new("Something went wrong."));
            }

            leptos_actix::redirect("/");

            Ok(())
        }
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

#[server]
pub async fn validate_user_session() -> Result<bool, ServerFnError> {
    let pool = extract::<Data<DbPool>>()
        .await
        .map_err(|_| ServerFnError::new("Database pool not found"))?;
    let mut conn = get_connection(&pool).map_err(|err| ServerFnError::new(err.to_string()))?;
    let session = extract::<Session>()
        .await
        .map_err(|_| ServerFnError::new("Session not found"))?;

    is_authenticated(&session, &mut conn)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
        .map(|_| true)
}
