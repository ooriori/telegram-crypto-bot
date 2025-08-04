use teloxide::{prelude::*, utils::command::BotCommands};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(BotCommands, Clone, Debug, Serialize, Deserialize)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Show this help message")]
    Help,

    #[command(description = "Get the current price of a cryptocurrency", parse_with = "split")]
    Price(String),
}

#[derive(Deserialize, Serialize, Debug)]
struct CoinGeckoPriceResponse(HashMap<String, HashMap<String, f64>>);

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

    log::info!("Starting telegram_crypto_bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(
    bot: Bot,
    msg: Message,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Price(coin) => {
            let price = get_coin_price(&coin).await;
            bot.send_message(msg.chat.id, format!("The current price of {} is ${:.4}", coin, price))
                .await?;
        }
    }
    Ok(())
}

async fn get_coin_price(coin: &str) -> f64 {
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", coin);
    let response: CoinGeckoPriceResponse = match reqwest::get(&url).await {
        Ok(resp) => match resp.json().await {
            Ok(json) => json,
            Err(_) => return 0.0,
        },
        Err(_) => return 0.0,
    };
    
    response.0.get(coin).and_then(|c| c.get("usd")).cloned().unwrap_or(0.0)
}
