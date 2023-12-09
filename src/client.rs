use crate::api::Api;

pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) token: String,
}

impl Client {
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.into(),
        }
    }

    pub fn api<A: Api>(self) -> A {
        A::from_client(self)
    }
}
