use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{api::prelude::*, database::schema::short_links, utils::b62encode};

pub async fn generate_shortlink(
    user_id: Option<uuid::Uuid>,
    anon_id: Option<uuid::Uuid>,
    payload: CreateShortLinkDto,
    conn: &mut DbConnection,
) -> Result<ShortLink> {
    let new_shortlink = NewShortLink {
        owner_id: user_id,
        anonymous_owner_id: anon_id,
        long_url: payload.url,
        status: ShortlinkStatus::Inactive.to_string(),
        is_private: true,
    };

    let (id, serial_id): (uuid::Uuid, i64) = insert_into(short_links::table)
        .values(new_shortlink)
        .returning((short_links::id, short_links::serial_id))
        .get_result(conn)?;

    let hash = b62encode(serial_id as usize + chrono::Utc::now().timestamp_millis() as usize);

    let result = update(short_links::table.find(id))
        .set(short_links::hash.eq(hash))
        .returning(ShortLink::as_returning())
        .get_result(conn)?;

    Ok(result)
}

pub async fn list_user_shortlinks(
    user_id: uuid::Uuid,
    conn: &mut DbConnection,
) -> Result<Vec<ShortLink>> {
    let data = short_links::table
        .filter(short_links::owner_id.eq(user_id))
        .select(ShortLink::as_select())
        .get_results(conn)?;

    Ok(data)
}

pub async fn get_long_url(hash: &String, conn: &mut DbConnection) -> Result<String> {
    short_links::table
        .filter(short_links::hash.eq(Some(hash)))
        .select(short_links::long_url)
        .get_result(conn)
        .map_err(|_| ApiError::NotFound)
}
