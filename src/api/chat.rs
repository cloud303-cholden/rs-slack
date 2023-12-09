use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

use crate::client::{FromClient, Client};

use super::Api;

pub struct Chat<'a> {
    client: &'a reqwest::Client,
    token: &'a str,
}

impl<'a> Chat<'a> {
    pub async fn post_message(
        &self,
        input: input::PostMessage<'a>,
    ) -> Result<output::PostMessage, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.append(AUTHORIZATION, format!("Bearer {}", self.token).parse().unwrap());
        headers.append(CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
        let resp: output::PostMessage = self.client
            .post("https://slack.com/api/chat.postMessage")
            .json(&input)
            .headers(headers)
            .send().await?
            .json().await?;
        Ok(resp)
    }
}

impl<'a> Api<'a> for Chat<'a> {
    fn endpoint() -> &'a str {
       "https://slack.com/api/chat.postMessage" 
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

pub mod input {
    #[derive(Default, serde::Serialize)]
    pub struct PostMessage<'a> {
        pub channel: &'a str,
        pub text: &'a str,
        pub thread_ts: Option<&'a str>,
    }
    pub type PostMessageInput<'a> = PostMessage<'a>;
}

pub mod output {
    use crate::api::chat::types::*;

    #[derive(Debug, serde::Deserialize)]
    pub struct PostMessage {
        pub ok: bool,
        pub channel: String,
        pub ts: String,
        pub message: Message,
        pub warning: Option<String>,
        pub response_metadata: Option<ResponseMetadata>,
    }
    pub type PostMessageOutput = PostMessage;
}

pub mod types {
    #[derive(Debug, serde::Deserialize)]
    pub struct Message {
        #[serde(rename(deserialize = "type"))]
        pub type_: String,
        pub bot_id: String,
        pub text: String,
        pub user: String,
        pub ts: String,
        pub app_id: String,
        pub blocks: Vec<Block>,
        pub team: String,
        pub bot_profile: BotProfile,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Block {
        #[serde(rename(deserialize = "type"))]
        pub type_: String,
        pub block_id: String,
        pub elements: Vec<Element>,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Element {
        #[serde(rename(deserialize = "type"))]
        pub type_: String,
        pub elements: Option<Vec<Element>>,
        pub text: Option<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct BotProfile {
        pub id: String,
        pub app_id: String,
        pub name: String,
        pub icons: Icons,
        pub deleted: bool,
        pub updated: u64,
        pub team_id: String,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Icons {
        pub image_36: String,
        pub image_48: String,
        pub image_72: String,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct ResponseMetadata {
        pub warnings: Vec<String>,
    }
}
