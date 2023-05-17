use dotenv::dotenv;
use log;
use pretty_env_logger;
use teloxide::{prelude::*};

mod telegram;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting Bhaktivedanta Swami Prabhupada chat bot...");

    let bot = Bot::from_env();
    teloxide::repl(bot, telegram::answer).await;
}
