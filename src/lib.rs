use chrono::prelude::*;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use base64::{encode, decode};
use url::{Url, Host, Position};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fmt::Error;


extern crate chrono;

enum MsgType {
    MarkDown,
    Text,
}

#[derive(Serialize, Deserialize)]
struct Data {
    msgtype: String,
    markdown: Markdown,
    #[serde(rename = "is_Atall")]
    is_atall: bool,
    at: At,
}


#[derive(Serialize, Deserialize)]
struct Markdown {
    title: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct At {
    #[serde(rename = "atMobiles")]
    atmobiles: Vec<String>,
    #[serde(rename = "atUserIds")]
    atuserids: Vec<String>,

}

#[derive(Clone, Debug)]
pub struct DingTalkBotClient {
    dingtalk_hook: String,
    dingtalk_sign: String,
    is_atall: bool,
    at: At,
}

impl DingTalkBotClient {
    pub fn new(dingtalk_hook: &str, dingtalk_sign: &str) -> DingTalkBotClient {
        DingTalkBotClient {
            dingtalk_hook: dingtalk_hook.into(),
            dingtalk_sign: dingtalk_sign.into(),
            is_atall: false,
            at: At {
                atmobiles: vec![],
                atuserids: vec![],
            },
        }
    }

    pub fn at_somebody_by_mobile(mut self, phone_number_list: Vec<String>) -> DingTalkBotClient {
        let newclient = DingTalkBotClient {
            at: At {
                atmobiles: phone_number_list,
                atuserids: vec![],
            },
            ..self
        };
        newclient
    }
    pub fn at_somebody_by_id(mut self, user_id_list: Vec<String>) -> DingTalkBotClient {
        let newclient = DingTalkBotClient {
            at: At {
                atmobiles: vec![],
                atuserids: user_id_list,
            },
            ..self
        };
        newclient
    }
    pub fn at_all(self) -> DingTalkBotClient {
        let newclient = DingTalkBotClient {
            is_atall: true,
            ..self
        };
        newclient
    }
    pub async fn send_msg(self, title: &str, content: &str) -> Result<String, Error> {
        type HmacSha256 = Hmac<Sha256>;
        let timestamp = Local::now().timestamp_millis();
        let string_to_sign = String::from(format!("{}\n{}", timestamp, self.dingtalk_sign));
        let mut mac = HmacSha256::new_from_slice(self.dingtalk_sign.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let sign = encode(code_bytes);
        let full_url = format!("{}&timestamp={}&sign={}", self.dingtalk_hook, timestamp, sign);
        let dingtalk_url = Url::parse(full_url.as_str()).unwrap();
        let data = &Data {
            msgtype: "markdown".to_string(),
            markdown: Markdown { title: title.into(), text: content.into() },
            is_atall: self.is_atall,
            at: self.at,
        };
        let res = surf::post(dingtalk_url).content_type("application/json").body(json!(data)).await;

        match res {
            Ok(mut response) => {
                match response.body_string().await {
                    Ok(text) => {
                        Ok(text)
                    }
                    Err(_) => {
                        Err(Error)
                    }
                }
            }
            Err(_) => {
                Err(Error)
            }
        }
    }
}

