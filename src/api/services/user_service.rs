use diesel::{dsl::exists, select, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{api::prelude::*, database::schema::users};

pub async fn get_user_by_id(user_id: uuid::Uuid, conn: &mut DbConnection) -> Result<UserProfile> {
    users::table
        .find(user_id)
        .select(UserProfile::as_select())
        .first(conn)
        .map_err(|_| ApiError::UserNotFound)
}

pub async fn is_user_exist(user_id: uuid::Uuid, conn: &mut DbConnection) -> Result<bool> {
    Ok(select(exists(users::table.filter(users::id.eq(user_id)))).get_result::<bool>(conn)?)
}
