use crate::models::{AuthUser, LoginUser, RegisterUser, User};
use crate::{verify_password, Pool};
use diesel::insert_into;
use diesel::prelude::*;
use secrecy::Secret;

pub fn login_user(pool: Pool, login_user: LoginUser) -> Option<AuthUser> {
    use crate::database::schema::users::dsl::*;
    let conn = &pool.get().unwrap();
    let result = users
        .filter(email.eq(&login_user.email))
        .first::<User>(conn);

    if let Some(user) = result.ok() {
        if verify_password(login_user.password, Secret::from(user.password_hash)).unwrap() {
            let result = AuthUser { email: user.email };

            return Some(result);
        }
    }

    return None;
}

pub fn register_user(pool: Pool, new_user: RegisterUser) -> Option<AuthUser> {
    use crate::database::schema::users::dsl::*;

    let conn = pool.get().unwrap();
    let inserted_user = insert_into(users)
        .values(new_user)
        .get_result::<User>(&conn);

    match inserted_user {
        Ok(user) => {
            let result = AuthUser { email: user.email };

            return Some(result);
        }
        Err(_) => return None,
    }
}
