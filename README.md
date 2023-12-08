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
    client.api::<Chat>().post_message(input::PostMessage {
        channel: "C0000000000",
        text: "Test",
        thread_ts: None,
    }).await?;
    Ok(())
}
```
