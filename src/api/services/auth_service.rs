use actix_session::Session;
use bcrypt::verify;
use diesel::{insert_into, prelude::*};
use secrecy::ExposeSecret;

use crate::{api::prelude::*, database::schema::users};

pub async fn sign_in(payload: LoginDto, conn: &mut DbConnection) -> Result<uuid::Uuid> {
    let user = users::table
        .select(AuthUser::as_select())
        .filter(users::email.eq(payload.email))
        .limit(1)
        .first(conn)
        .map_err(|_| AuthError::InvalidCredentials)?;

    let is_match = verify(payload.password.expose_secret(), &user.password_hash)?;

    if is_match {
        return Ok(user.id);
    }

    Err(ApiError::InvalidCredentials)
}

pub async fn sign_up(payload: RegisterDto, conn: &mut DbConnection) -> Result<uuid::Uuid> {
    let new_user = RegisterUser::try_from(payload)?;
    let user = insert_into(users::table)
        .values(new_user)
        .returning(AuthUser::as_returning())
        .get_result(conn)?;

    Ok(user.id)
}

pub async fn is_authenticated(session: &Session, conn: &mut DbConnection) -> Result<uuid::Uuid> {
    let user_id = match session.get::<uuid::Uuid>("session") {
        Ok(Some(id)) => id,
        Ok(None) => return Err(ApiError::Unauthorized),
        Err(_) => return Err(ApiError::InternalError),
    };

    let is_exist = is_user_exist(user_id, conn).await?;

    if !is_exist {
        return Err(ApiError::Unauthorized);
    }

    Ok(user_id)
}

pub async fn is_anonymous(session: &Session) -> Option<uuid::Uuid> {
    session.get::<uuid::Uuid>("guest_session").ok().flatten()
}
