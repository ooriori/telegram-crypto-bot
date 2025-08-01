use teloxide::{prelude::*, utils::command::BotCommands};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Estos son los comandos disponibles:")]
enum Command {
    #[command(description = "Mostrar esta ayuda.")]
    Help,

    #[command(description = "Obtener el precio actual de Bitcoin en USD.")]
    Price,

    #[command(description = "Obtener el precio actual de Ethereum en USD.")]
    EthPrice,

    #[command(description = "Obtener el precio actual de Dogecoin en USD.")]
    DogePrice,

    #[command(description = "Obtener el precio actual de Litecoin en USD.")]
    LtcPrice,

    #[command(description = "Obtener el precio actual de Ripple (XRP) en USD.")]
    XrpPrice,
}

#[derive(Deserialize, Debug)]
struct CoinGeckoPriceResponse(HashMap<String, HashMap<String, f64>>);

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Crypto Bot is starting...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(
    bot: Bot,
    message: Message,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => {
            // Convertimos explícitamente a String con to_string()
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Price => {
            let price = get_price("bitcoin").await.unwrap_or(-1.0);
            bot.send_message(message.chat.id, format!("Precio Bitcoin: ${:.2}", price)).await?;
        }
        Command::EthPrice => {
            let price = get_price("ethereum").await.unwrap_or(-1.0);
            bot.send_message(message.chat.id, format!("Precio Ethereum: ${:.2}", price)).await?;
        }
        Command::DogePrice => {
            let price = get_price("dogecoin").await.unwrap_or(-1.0);
            bot.send_message(message.chat.id, format!("Precio Dogecoin: ${:.4}", price)).await?;
        }
        Command::LtcPrice => {
            let price = get_price("litecoin").await.unwrap_or(-1.0);
            bot.send_message(message.chat.id, format!("Precio Litecoin: ${:.2}", price)).await?;
        }
        Command::XrpPrice => {
            let price = get_price("ripple").await.unwrap_or(-1.0);
            bot.send_message(message.chat.id, format!("Precio Ripple (XRP): ${:.4}", price)).await?;
        }
    }
    Ok(())
}

async fn get_price(coin: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        coin
    );
    let response: CoinGeckoPriceResponse = reqwest::get(&url).await?.json().await?;

    if let Some(coin_map) = response.0.get(coin) {
        if let Some(price) = coin_map.get("usd") {
            Ok(*price)
        } else {
            Err("Precio USD no encontrado".into())
        }
    } else {
        Err("Criptomoneda no encontrada".into())
    }
}
