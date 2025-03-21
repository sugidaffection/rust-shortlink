use diesel::prelude::*;
use serde::Serialize;

use crate::database::schema::users;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
}
