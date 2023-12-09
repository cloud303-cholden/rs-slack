pub mod chat;
pub use chat::Chat;

pub mod input {
    pub use crate::api::chat::input::*;
}

pub trait Api<'a> {
    fn endpoint() -> &'a str;
}
