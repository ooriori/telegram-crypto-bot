# Telegram Crypto Price Bot (Rust)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Descripción

Bot de Telegram desarrollado en Rust usando `teloxide` que provee precios en tiempo real de criptomonedas populares como Bitcoin, Ethereum, Dogecoin, Litecoin y Ripple.

## Características

- Comandos fáciles de usar para consultar precios.
- Integración con la API pública de CoinGecko.
- Código seguro y eficiente con Rust y Tokio.
- Diseño modular y escalable para futuras mejoras.

## Comandos disponibles

| Comando  | Descripción                           |
| -------- | ----------------------------------- |
| `/help`  | Muestra esta guía de comandos.      |
| `/price` | Precio actual de Bitcoin en USD.    |
| `/ethprice` | Precio actual de Ethereum en USD. |
| `/dogeprice` | Precio actual de Dogecoin en USD. |
| `/ltcprice` | Precio actual de Litecoin en USD. |
| `/xrpprice` | Precio actual de Ripple (XRP) en USD. |

## Requisitos

- Rust (edición 2021)
- Token de bot Telegram (obtenido con BotFather)

## Instalación y uso

1. Clonar el repositorio:

```bash
git clone https://github.com/tu_usuario/telegram-crypto-bot.git
cd telegram-crypto-bot
