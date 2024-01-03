use weworkbot_rs::{
    message::{MessageMarkdown, MessageText},
    ClientBuilder,
};

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new("your key").build();
    let msg = MessageText::new("Hello, world!".into(), vec![], vec![]);
    let res = client.send(msg).await;
    match res {
        Ok(_) => println!("Send text message successfully!"),
        Err(e) => println!("Send text message failed: {}", e),
    }

    let markdown_msg = MessageMarkdown::new(
        "实时新增用户反馈<font color=\"warning\">132例</font>，请相关同事注意。\n".into(),
    );
    let res2 = client.send(markdown_msg).await;
    match res2 {
        Ok(_) => println!("Send markdown message successfully!"),
        Err(e) => println!("Send markdown message failed: {}", e),
    }
}
