pub mod chat;

use std::sync::Arc;

pub use chat::Chat;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use async_trait::async_trait;

use crate::Client;

pub mod input {
    pub use crate::api::chat::input::*;
}

struct Request<A, S>
where
    A: Api,
    S: serde::Serialize,
{
    api: A,
    input: Option<S>,
    endpoint: String,
}

impl<A, S> Request<A, S>
where
    A: Api,
    S: serde::Serialize,
{
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append(
            AUTHORIZATION,
            format!("Bearer {}", self.api.client().token).parse().unwrap(),
        );
        headers.append(
            CONTENT_TYPE,
            "application/json; charset=utf-8".parse().unwrap(),
        );
        headers
    }

    async fn post<D>(self) -> Result<D, reqwest::Error>
    where D: serde::de::DeserializeOwned
    {
        let headers = self.headers();
        let resp = self
            .api
            .client()
            .client
            .post(self.endpoint)
            .json(&self.input.unwrap())
            .headers(headers)
            .send().await?
            .json().await?;
        Ok(resp)
    }
}

#[async_trait]
pub trait Api {
    fn client(&self) -> Arc<Client>;
    fn from_client(client: Client) -> Self;
}
