use std::fmt;

use crate::prelude::{DbConnection, DbPool};

use super::prelude::*;
use bcrypt::hash;
use secrecy::ExposeSecret;

pub fn get_connection(pool: &DbPool) -> Result<DbConnection> {
    pool.get().map_err(|err| ApiError::DbConnectionError(err))
}

impl TryFrom<RegisterDto> for RegisterUser {
    type Error = ApiError;

    fn try_from(dto: RegisterDto) -> Result<Self> {
        let password_hash = hash(dto.password.expose_secret(), 12)?;
        Ok(Self {
            password_hash,
            username: dto.username,
            email: dto.email,
        })
    }
}

pub enum ShortlinkStatus {
    Active,
    Inactive,
    Expired,
}

impl fmt::Display for ShortlinkStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShortlinkStatus::Active => "active",
                ShortlinkStatus::Inactive => "inactive",
                ShortlinkStatus::Expired => "expired",
            }
        )
    }
}
