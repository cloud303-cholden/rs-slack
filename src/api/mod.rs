pub mod chat;

use std::sync::Arc;

pub use chat::Chat;
use async_trait::async_trait;

use crate::client::ClientInner;

pub mod input {
    pub use crate::api::chat::input::*;
}

pub(crate) struct Request<'a, S>
where
    S: serde::Serialize,
{
    pub(crate) input: Option<&'a S>,
    pub(crate) endpoint: String,
}

#[async_trait]
pub trait Api {
    fn client(&self) -> Arc<ClientInner>;
    fn from_client(client: Arc<ClientInner>) -> Self;
}
