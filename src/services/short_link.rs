use std::fmt;

use crate::models::{IdAndSerialId, ShortLink};
use crate::{
    database::schema::short_links,
    models::{LongURL, NewShortLink},
};
use crate::{errors, prelude::*};
use diesel::{insert_into, prelude::*, update};
use uuid::Uuid;

enum Status {
    Active,
    Inactive,
    Expired,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Active => "active",
                Status::Inactive => "inactive",
                Status::Expired => "expired",
            }
        )
    }
}

pub struct ShortLinkService;

impl ShortLinkService {
    pub fn create(
        conn: &mut PConn,
        values: LongURL,
        user_id: Uuid,
    ) -> Result<ShortLink, errors::ShortLinkError> {
        let input = NewShortLink {
            long_url: values.url,
            owner_id: user_id,
            is_private: true,
            status: Status::Inactive.to_string(),
            title: values.title,
            description: values.description,
        };

        let data: IdAndSerialId = insert_into(short_links::table)
            .values(input)
            .returning(IdAndSerialId::as_returning())
            .get_result(conn)?;

        let hashed_url =
            b62encode(data.serial_id as usize + chrono::offset::Local::now().timestamp() as usize)
                .unwrap();

        let result: ShortLink = update(short_links::table.find(data.id))
            .set(short_links::hash.eq(hashed_url))
            .returning(ShortLink::as_returning())
            .get_result(conn)?;

        Ok(result)
    }

    pub fn get(conn: &mut PConn, code: String) -> Option<String> {
        use crate::database::schema::short_links;
        let result = short_links::dsl::short_links
            .select(short_links::dsl::long_url)
            .filter(short_links::dsl::hash.eq(code))
            .limit(1)
            .first(conn);
        result.ok()
    }

    pub fn get_all(
        conn: &mut PConn,
        user_id: Uuid,
    ) -> Result<Vec<ShortLink>, errors::ShortLinkError> {
        use crate::database::schema::short_links;
        let result = short_links::dsl::short_links
            .filter(short_links::dsl::owner_id.eq(user_id))
            .get_results(conn)?;

        Ok(result)
    }
}
