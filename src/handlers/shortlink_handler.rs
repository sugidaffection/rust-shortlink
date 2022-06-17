use crate::models::{NewShortLink, ShortLink};
use crate::{b62encode, LongURLForm, Pool};
use diesel::insert_into;
use diesel::prelude::*;

pub fn create_short_link(pool: Pool, values: LongURLForm, user_session: String) -> Option<String> {
    use crate::database::schema::short_link;
    use crate::database::schema::users;
    let conn = &pool.get().unwrap();
    let user_id = users::dsl::users
        .select(users::dsl::id)
        .filter(users::dsl::email.eq(user_session))
        .limit(1)
        .first(conn)
        .unwrap();
    let last_item = short_link::dsl::short_link
        .select(short_link::dsl::uid)
        .order(short_link::dsl::uid.desc())
        .limit(1)
        .first(conn)
        .unwrap_or(1);
    let short_code =
        b62encode(last_item as usize + (chrono::offset::Local::now().timestamp() as usize))
            .unwrap();
    let input = NewShortLink {
        long_url: values.url,
        owner_id: user_id,
        hash: short_code,
    };
    let insert = insert_into(short_link::dsl::short_link)
        .values(input)
        .get_result::<ShortLink>(conn);

    insert.map_or(None, |x: ShortLink| Some(x.hash))
}

pub fn get_short_link(pool: Pool, code: String) -> Option<String> {
    use crate::database::schema::short_link;
    let conn = &pool.get().unwrap();
    let result = short_link::dsl::short_link
        .select(short_link::dsl::long_url)
        .filter(short_link::dsl::hash.eq(code))
        .limit(1)
        .first(conn);
    result.ok()
}
