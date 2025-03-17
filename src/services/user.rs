use crate::prelude::*;
use crate::{database::schema::users, models::User};
use diesel::{delete, prelude::*};
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub fn get_user_by_id(conn: &mut PConn, user_id: Uuid) -> Result<User, diesel::result::Error> {
        users::table.find(user_id).first(conn)
    }

    pub fn get_one_by_email(
        conn: &mut PConn,
        email_: String,
    ) -> Result<User, diesel::result::Error> {
        users::table
            .filter(users::email.eq(email_))
            .limit(1)
            .first(conn)
    }

    pub fn delete_by_email(conn: &mut PConn, email_: String) -> bool {
        let result = delete(users::table.filter(users::email.eq(email_))).execute(conn);
        if let Some(count) = result.ok() {
            if count > 0 {
                return true;
            }
        }

        false
    }
}
