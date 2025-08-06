use teloxide::{prelude::*, utils::command::BotCommands};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;
use teloxide::types::MessageKind;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Comandos disponibles:")]
pub enum Command {
    Help,
    Price(String),
    Analyze(String),
}

#[derive(Deserialize, Serialize, Debug)]
struct CoinGeckoPriceResponse(HashMap<String, HashMap<String, f64>>);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    log::info!("Iniciando telegram_crypto_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        if let Some(text) = message.update.text() {
            // Si el mensaje empieza con "/" -> Es un comando
            if text.starts_with('/') {
                // Intenta parsear el comando
                match Command::parse(text, "telegram_crypto_bot") {
                    Ok(command) => {
                        answer(message.requester.clone(), message.update.clone(), command).await?;
                    }
                    Err(_) => {
                        // Comando no reconocido
                        message.answer("❌ Comando no reconocido. Usa /help para ver los comandos disponibles.")
                            .send()
                            .await?;
                    }
                }
            } else {
                // Mensajes que no son comandos, opcionalmente responder algo
                message.answer("Hola! Usa /help para ver los comandos disponibles.")
                    .send()
                    .await?;
            }
        }
        respond(())
    })
    .await;
}

async fn answer(bot: Bot, msg: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Price(coin) => {
            let coin = coin.to_lowercase();

            match get_coin_price(&coin).await {
                Ok(price) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("📈 El precio actual de *{}* es *${:.4}*", coin, price),
                    )
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;
                }
                Err(err_msg) => {
                    bot.send_message(msg.chat.id, err_msg).await?;
                }
            }
        }
        Command::Analyze(coin) => {
            let coin = coin.to_lowercase();
            bot.send_message(msg.chat.id, "🧠 Analizando el mercado con IA...")
                .await?;

            match analyze_coin_with_gpt(&coin).await {
                Ok(analysis) => {
                    bot.send_message(msg.chat.id, analysis)
                        .await?;
                }
                Err(err_msg) => {
                    bot.send_message(msg.chat.id, err_msg).await?;
                }
            }
        }
    }
    Ok(())
}

async fn get_coin_price(coin: &str) -> Result<f64, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        coin
    );

    let response: CoinGeckoPriceResponse = reqwest::get(&url)
        .await
        .map_err(|_| "Error al contactar la API de CoinGecko.".to_string())?
        .json()
        .await
        .map_err(|_| "Error al procesar la respuesta de CoinGecko.".to_string())?;

    response
        .0
        .get(coin)
        .and_then(|c| c.get("usd"))
        .cloned()
        .ok_or_else(|| {
            format!(
                "⚠️ No se encontró el precio para '{}'. Asegúrate de usar el nombre en inglés y en minúsculas, por ejemplo: bitcoin, ethereum, dogecoin.",
                coin
            )
        })
}

async fn analyze_coin_with_gpt(coin: &str) -> Result<String, String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "🔒 No se encontró la API key de OpenAI. Define OPENAI_API_KEY en tus variables de entorno.".to_string())?;

    let prompt = format!(
        "Haz un análisis simple del estado actual del mercado de la criptomoneda '{}'. Sé claro, breve y sin tecnicismos.",
        coin
    );

    let client = Client::new();
    let res = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7,
        }))
        .send()
        .await
        .map_err(|_| "❌ Error al contactar OpenAI.".to_string())?;

    let json: serde_json::Value = res.json()
        .await
        .map_err(|_| "❌ Error al procesar respuesta de OpenAI.".to_string())?;

    let message = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "⚠️ No se pudo leer la respuesta de la IA.".to_string())?;

    Ok(message.to_string())
}