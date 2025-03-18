use diesel::prelude::*;

use crate::database::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct RegisterUser {
    pub password_hash: String,
    pub username: String,
    pub email: String,
}
