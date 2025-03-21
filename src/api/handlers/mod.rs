mod auth_handler;
mod shortlink_handler;
mod user_handler;

pub use auth_handler::{login, register};
pub use shortlink_handler::{create_shortlink, get_shortlinks, redirect};
pub use user_handler::profile;
