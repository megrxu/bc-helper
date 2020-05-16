use bchelper_lib::helper::*;
use clap::Clap;
use futures::StreamExt;
use std::env;
use telegram_bot::*;
use tokio::io::AsyncWriteExt;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Ray Xu. <megrxu@gmail.com>")]
struct Opts {
    /// config file
    #[clap(short, long, default_value = "config.yml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let auth = env::var("ADMIN_USER")
        .expect("ADMIN_USER not set")
        .parse::<i64>()
        .expect("ADMIN_USER not valid");
    let api = Api::new(token);

    let opts: Opts = Opts::parse();
    let config_path = opts.config;
    let instance = HelperInstance::new(&config_path);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        match update.kind {
            UpdateKind::Message(message) => {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    if message.from.id == UserId::new(auth) {
                        match instance.parse(data) {
                            Ok(result) => {
                                let mut reply =
                                    message.text_reply(format!("{}", result.markdown()));
                                api.send(SendChatAction::new(message.chat, ChatAction::Typing))
                                    .await?;
                                api.send(reply.parse_mode(ParseMode::Markdown)).await?;
                                let mut file = tokio::fs::OpenOptions::new()
                                    .append(true)
                                    .open(&instance.default_config.operating_file)
                                    .await
                                    .unwrap();
                                file.write_all(&result.export().bytes().collect::<Vec<u8>>()).await.unwrap();
                            }
                            Err(e) => {
                                api.send(message.text_reply(format!("{}", e))).await?;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
