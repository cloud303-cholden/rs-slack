use std::sync::Arc;
use std::ops::Deref;

use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

use crate::api::{Api, Request};

pub struct Client {
    inner: Arc<ClientInner>,
}

impl Deref for Client {
    type Target = ClientInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct ClientInner {
    pub(crate) client: reqwest::Client,
    pub(crate) token: String,
}

impl Client {
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            inner: Arc::new(ClientInner {
                client: reqwest::Client::new(),
                token: token.into(),
            })
        }
    }

    pub fn api<A: Api>(&self) -> A {
        A::from_client(self.inner.clone())
    }

}

impl ClientInner {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append(
            AUTHORIZATION,
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        headers.append(
            CONTENT_TYPE,
            "application/json; charset=utf-8".parse().unwrap(),
        );
        headers
    }

    pub(crate) async fn post<D, S>(self: Arc<Self>, request: Request<'_, S>) -> Result<D, reqwest::Error>
    where
        D: serde::de::DeserializeOwned,
        S: serde::Serialize,
    {
        let headers = self.headers();
        let resp = self
            .client
            .post(request.endpoint)
            .json(&request.input.unwrap())
            .headers(headers)
            .send().await?
            .json().await?;
        Ok(resp)
    }
}
