use log::info;
use teloxide::{prelude::*, utils::command::BotCommands};

mod openai;

#[derive(teloxide::utils::command::BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
pub enum Command {
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "show supported commands.")]
    Help,
    #[command(description = "delete previous conversation.")]
    Clear,
}

pub async fn answer(bot: Bot, msg: Message) -> ResponseResult<()> {
    let message = msg.text().unwrap_or_default().to_string();
    let username = msg.from().and_then(|user| user.username.clone()).unwrap_or_else(|| String::from("unknown"));

    info!("@{}: {}", username, message);

    if message.starts_with('/') {
        let response = match Command::parse(&message, "") {
            Ok(command) => match command {
                Command::Start => format!("Hare Krishna, @{username}!"),
                Command::Help => Command::descriptions().to_string(),
                Command::Clear => "Clear conversation history. Coming soon...".to_string(),
            },
            Err(_) => message,
        };

        if !response.is_empty() {
            bot.send_message(msg.chat.id, response).await?;
        }
    } else {
        if !message.is_empty() {
            bot.send_message(msg.chat.id, openai::gpt(message).await).await?;
        }
    }

    Ok(())
}