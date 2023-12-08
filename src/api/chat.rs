use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

use crate::client::{FromClient, Client};

pub struct Chat<'a> {
    client: &'a reqwest::Client,
    token: &'a str,
}

impl<'a> Chat<'a> {
    pub async fn post_message(
        &self,
        input: PostMessageInput<'a>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let body = json!({"text": input.text, "channel": input.channel, "thread_ts": input.thread_ts});
        let mut headers = HeaderMap::new();
        headers.append(AUTHORIZATION, format!("Bearer {}", self.token).parse().unwrap());
        headers.append(CONTENT_TYPE, "application/json".parse().unwrap());
        self.client
            .post("https://slack.com/api/chat.postMessage")
            .json(&body)
            .headers(headers)
            .send()
            .await
    }
}

impl<'a> FromClient<'a> for Chat<'a> {
    fn from_client(client: &'a Client) -> Self {
        Self {
            client: &client.client,
            token: client.token,
        }
    }
}

#[derive(Default)]
pub struct PostMessageInput<'a> {
    pub channel: &'a str,
    pub text: &'a str,
    pub thread_ts: Option<&'a str>,
}
