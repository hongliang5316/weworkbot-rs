use crate::message::{Message, MessageMarkdown, MessageText};
use serde::{Deserialize, Serialize};

static BASE_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook";

#[derive(Debug, Deserialize)]
struct Response {
    errcode: i32,
    errmsg: String,
}

pub struct Client {
    inner: reqwest::Client,
    key: String,
}

pub struct ClientBuilder {
    key: String,
}

impl ClientBuilder {
    pub fn new(key: &str) -> Self {
        Self { key: key.into() }
    }

    pub fn build(self) -> Client {
        Client {
            inner: reqwest::Client::new(),
            key: self.key,
        }
    }
}

impl Client {
    pub async fn send<T>(&self, msg: T) -> crate::Result<()>
    where
        T: Message + Serialize,
    {
        let url = format!("{}/send?key={}", BASE_URL, self.key);
        let resp = self.inner.post(&url).json(&msg).send().await?;
        if resp.status().is_success() {
            let resp: Response = resp.json().await?;
            if resp.errcode != 0 {
                return Err(resp.errmsg.into());
            }
            Ok(())
        } else {
            Err(Box::new(resp.error_for_status().unwrap_err()))
        }
    }
}
