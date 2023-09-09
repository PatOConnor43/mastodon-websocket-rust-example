use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_json::{json, Value};
use std::error::Error;
use tungstenite::connect;
use url::Url;

#[derive(Deserialize, Debug)]
struct DirectMessageEvent<'a> {
    #[serde(borrow)]
    payload: &'a RawValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessagePayload {
    pub id: String,
    pub unread: bool,
    pub accounts: Vec<Account>,
    #[serde(rename = "last_status")]
    pub last_status: LastStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    pub group: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    #[serde(rename = "avatar_static")]
    pub avatar_static: String,
    pub header: String,
    #[serde(rename = "header_static")]
    pub header_static: String,
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    #[serde(rename = "following_count")]
    pub following_count: i64,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    #[serde(rename = "last_status_at")]
    pub last_status_at: String,
    pub emojis: Vec<Value>,
    pub fields: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastStatus {
    pub id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "in_reply_to_id")]
    pub in_reply_to_id: Value,
    #[serde(rename = "in_reply_to_account_id")]
    pub in_reply_to_account_id: Value,
    pub sensitive: bool,
    #[serde(rename = "spoiler_text")]
    pub spoiler_text: String,
    pub visibility: String,
    pub language: Value,
    pub uri: String,
    pub url: String,
    #[serde(rename = "replies_count")]
    pub replies_count: i64,
    #[serde(rename = "reblogs_count")]
    pub reblogs_count: i64,
    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,
    #[serde(rename = "edited_at")]
    pub edited_at: Value,
    pub favourited: bool,
    pub reblogged: bool,
    pub muted: bool,
    pub bookmarked: bool,
    pub content: String,
    pub filtered: Vec<Value>,
    pub reblog: Value,
    pub account: Account,
    #[serde(rename = "media_attachments")]
    pub media_attachments: Vec<Value>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Value>,
    pub emojis: Vec<Value>,
    pub card: Value,
    pub poll: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let domain = std::env::var("STREAMING_DOMAIN").unwrap();
    let stream = std::env::var("STREAM").unwrap();
    let token = std::env::var("ACCESS_TOKEN").unwrap();

    let u = Url::parse(&format!(
        "{}/api/v1/streaming?stream={}&access_token={}",
        domain, stream, token
    ))
    .unwrap();
    let (mut socket, response) = connect(u).unwrap();
    dbg!(response);
    loop {
        let msg = socket.read().unwrap();
        if msg.is_text() {
            let text = msg.into_text().unwrap();
            let event = serde_json::from_str::<DirectMessageEvent>(text.as_str()).unwrap();
            dbg!(&event.payload);
            let deserialized_payload = json!(&event.payload);
            let deserialized_payload = serde_json::from_str::<DirectMessagePayload>(
                deserialized_payload.as_str().unwrap(),
            )
            .unwrap();
            dbg!(&deserialized_payload);
        }
    }
}
