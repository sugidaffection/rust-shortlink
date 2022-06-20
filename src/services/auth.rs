use diesel::{insert_into, prelude::*};
use secrecy::Secret;

use crate::{
    errors,
    forms::{LoginForm, RegisterForm},
    models::{AuthUser, NewUser, User},
    verify_password, PConn,
};

use super::user::UserService;

pub struct AuthService;

impl AuthService {
    pub fn login(conn: &PConn, form: LoginForm) -> Result<AuthUser, errors::AuthError> {
        let result = UserService::get_one_by_email(conn, form.email).map_or(None, |x| Some(x));

        if let Some(user) = result {
            if verify_password(form.password, Secret::from(user.password_hash)).unwrap() {
                return Result::Ok(AuthUser { email: user.email });
            }
        }

        return Result::Err(errors::AuthError::new(
            "Invalid username or password".to_owned(),
            "auth/invalid".to_owned(),
        ));
    }

    pub fn register(conn: &PConn, form: RegisterForm) -> Result<AuthUser, errors::AuthError> {
        use crate::database::schema::users::dsl::*;

        let result = UserService::get_one_by_email(&conn, form.email.clone());

        if let Some(user) = result.ok() {
            return Result::Err(errors::AuthError::new(
                "Email already used.".to_owned(),
                "auth/duplicate_email".to_owned(),
            ));
        }

        let new_user = NewUser::from(form);
        let inserted_user = insert_into(users).values(new_user).get_result::<User>(conn);

        match inserted_user {
            Ok(user) => {
                let result = AuthUser { email: user.email };

                return Result::Ok(result);
            }
            Err(_) => {
                return Result::Err(errors::AuthError::new(
                    "Failed to register new user".to_owned(),
                    "auth/request_error".to_owned(),
                ))
            }
        }
    }
}
