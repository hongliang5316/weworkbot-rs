use serde::{Deserialize, Serialize};

pub trait Message {
    fn msgtype(&self) -> MessageType;
}

impl Message for MessageText {
    fn msgtype(&self) -> MessageType {
        self.msgtype.clone()
    }
}

impl Message for MessageMarkdown {
    fn msgtype(&self) -> MessageType {
        self.msgtype.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum MessageType {
    TEXT,
    MARKDOWN,
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            MessageType::TEXT => serializer.serialize_str("text"),
            MessageType::MARKDOWN => serializer.serialize_str("markdown"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Text {
    content: String,
    mentioned_list: Vec<String>,
    mentioned_mobile_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageText {
    msgtype: MessageType,
    text: Text,
}

impl MessageText {
    pub fn new(
        content: String,
        mentioned_list: Vec<String>,
        mentioned_mobile_list: Vec<String>,
    ) -> Self {
        Self {
            msgtype: MessageType::TEXT,
            text: Text {
                content: content.into(),
                mentioned_list,
                mentioned_mobile_list,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageMarkdown {
    msgtype: MessageType,
    markdown: Markdown,
}

#[derive(Debug, Serialize, Deserialize)]
struct Markdown {
    content: String,
}

impl MessageMarkdown {
    pub fn new(content: String) -> Self {
        Self {
            msgtype: MessageType::MARKDOWN,
            markdown: Markdown {
                content: content.into(),
            },
        }
    }
}
