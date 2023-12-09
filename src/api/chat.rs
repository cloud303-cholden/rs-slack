use crate::client::ClientArc;

use super::{Api, Request};

pub struct Chat(ClientArc);

impl Chat {
    pub async fn post_message<'a, T>(&self, input: T) -> super::Result<output::PostMessage>
    where
        T: AsRef<input::PostMessage<'a>>,
    {
        let request = Request {
            input: Some(input.as_ref()),
            endpoint: "https://slack.com/api/chat.postMessage",
        };
        self.client().post(request).await
    }
}

impl Api for Chat {
    fn client(&self) -> ClientArc {
        self.0.clone()
    }

    fn from_client(client: ClientArc) -> Self {
        Self(client)
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

    impl<'a> AsRef<PostMessage<'a>> for PostMessage<'a> {
        fn as_ref(&self) -> &PostMessage<'a> {
            self
        }
    }
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
