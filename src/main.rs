use bchelper_lib::helper::*;
use futures::StreamExt;
use std::env;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let auth = env::var("ADMIN_USER")
        .expect("ADMIN_USER not set")
        .parse::<i64>()
        .expect("ADMIN_USER not valid");
    let api = Api::new(token);
    let config_path = env::args().nth(1).unwrap_or("./config.yml".to_string());
    let instance = HelperInstance::new(&config_path);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                if message.from.id == UserId::new(auth) {
                    match instance.parse(data) {
                        Ok(result) => {
                            let mut reply = message.text_reply(format!("```{}```", result));
                            api.send(reply.parse_mode(ParseMode::Markdown)).await?;
                        }
                        Err(e) => {
                            api.send(message.text_reply(format!("{}", e))).await?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
