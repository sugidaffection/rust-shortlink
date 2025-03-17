use crate::{database::schema::users, prelude::*};
use diesel::{insert_into, prelude::*};
use secrecy::SecretString;

use crate::{
    errors,
    forms::{LoginForm, RegisterForm},
    models::{AuthUser, NewUser},
};

use super::user::UserService;

pub struct AuthService;

impl AuthService {
    pub fn login(conn: &mut PConn, form: LoginForm) -> Result<AuthUser, errors::SignInError> {
        if let Some(user) = UserService::get_one_by_email(conn, form.email).ok() {
            if verify_password(form.password, SecretString::from(user.password_hash))
                .unwrap_or(false)
            {
                return Ok(AuthUser { id: user.id });
            }
        }

        Err(errors::SignInError::InvalidCredentials)
    }

    pub fn register(conn: &mut PConn, form: RegisterForm) -> Result<AuthUser, errors::SignUpError> {
        let result = UserService::get_one_by_email(conn, form.email.clone()).ok();

        if let Some(_) = result {
            return Result::Err(errors::SignUpError::DuplicateEmail);
        }

        let password_hash =
            hash_password(form.password).ok_or_else(|| errors::SignUpError::HashingFailed)?;

        let new_user = NewUser {
            username: form.username,
            email: form.email,
            password_hash,
        };

        let inserted_user = insert_into(users::table)
            .values(new_user)
            .returning(users::id)
            .get_result::<uuid::Uuid>(conn)
            .map(|value| AuthUser { id: value })
            .map_err(|_| errors::SignUpError::RegistrationFailed);

        inserted_user
    }
}
