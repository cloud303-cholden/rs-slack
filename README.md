## Slack SDK
An unofficial Slack SDK for Rust. This library is not affiliated with Slack. It is for personal use and experimentation. Use at your own risk.

## Example
```rust
use slack::{
    Client,
    api::{
        Chat,
        input,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("xoxb-000000000000-0000000000-000000000000000000000000");

    // API clients and inputs can be created and reused.
    let chat = client.api::<Chat>();
    let post_message = input::PostMessage {
        channel: "C0000000000",
        text: "Test",
        thread_ts: None,
    };
    // Inputs can be passed by value and by reference.
    chat.post_message(&post_message).await?;
    chat.post_message(post_message).await?;

    // Alternatively, make the entire request in a single statement.
    client
        .api::<Chat>()
        .post_message(input::PostMessage {
            channel: "C0000000000",
            text: "Test",
            thread_ts: None,
        }).await?
    Ok(())
}
```
