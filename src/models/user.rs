use crate::{database::schema::users, hash_password, PConn};
use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::forms::RegisterForm;

#[derive(Queryable, Deserialize, Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn get_one_by_email(
        conn: &PConn,
        email_data: String,
    ) -> Result<Self, diesel::result::Error> {
        use crate::database::schema::users::dsl::{email, users};
        users.filter(email.eq(email_data)).limit(1).first(conn)
    }

    pub fn create(conn: &PConn, data: NewUser) -> Result<Self, diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        insert_into(users).values(data).get_result::<Self>(conn)
    }
}

#[derive(Debug, Insertable, Deserialize, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn from(form: RegisterForm) -> Self {
        Self {
            username: form.username,
            email: form.email,
            password_hash: hash_password(form.password).expect("Failed to hash password"),
        }
    }
}
