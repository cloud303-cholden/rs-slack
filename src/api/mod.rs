pub mod chat;

pub use chat::Chat;
use async_trait::async_trait;

use crate::client::ClientArc;

pub mod input {
    pub use crate::api::chat::input::*;
}

pub(crate) struct Request<'a, S>
where
    S: serde::Serialize,
{
    pub(crate) input: Option<&'a S>,
    pub(crate) endpoint: &'a str,
}

#[allow(type_alias_bounds)]
pub type Result<D: serde::de::DeserializeOwned> = std::result::Result<D, reqwest::Error>;

#[async_trait]
pub trait Api {
    fn client(&self) -> ClientArc;
    fn from_client(client: ClientArc) -> Self;
}
