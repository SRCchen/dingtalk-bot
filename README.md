# lib for dingtalk bot


```toml
[dependencies]
dingtalkbot = {git = "https://github.com/SRCchen/dingtalk-bot" }
```
```rust
use dingtalkbot;


#[async_std::main]
async fn main() {
  let client = dingtalkbot::DingTalkBotClient::new("YOUR  DINGTALK URL","YOUR DINGTALK SECRET");
  let res = client.send_msg("YOUR MESSAGE TITLE","YOUR MESSAGE CONTEXT").await.unwrap();
  println!("{}",res)
}



```
