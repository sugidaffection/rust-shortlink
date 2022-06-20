use crate::{models::User, PConn};
use diesel::{delete, prelude::*};

pub struct UserService;

impl UserService {
    pub fn get_one_by_email(conn: &PConn, email_: String) -> Result<User, diesel::result::Error> {
        use crate::database::schema::users::dsl::{email, users};
        users.filter(email.eq(email_)).limit(1).first(conn)
    }

    pub fn delete_by_email(conn: &PConn, email_: String) -> bool {
        use crate::database::schema::users::dsl::{email, users};

        let result = delete(users.filter(email.eq(email_))).execute(conn);
        if let Some(count) = result.ok() {
            if count > 0 {
                return true;
            }
        }

        false
    }
}
