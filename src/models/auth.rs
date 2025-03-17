use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Queryable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct AuthUser {
    pub id: Uuid,
}
