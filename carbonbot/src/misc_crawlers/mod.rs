use std::sync::mpsc::Sender;

use crypto_crawler::{MarketType, Message};

mod binance;
mod bitmex;
mod bybit;
mod coinbase_pro;
mod huobi;

mod utils;

pub fn crawl_other(
    exchange: &str,
    market_type: MarketType,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => binance::crawl_other(market_type, tx, duration),
        "bitmex" => bitmex::crawl_other(market_type, tx, duration),
        "bybit" => bybit::crawl_other(market_type, tx, duration),
        "coinbase_pro" => coinbase_pro::crawl_other(market_type, tx, duration),
        "huobi" => huobi::crawl_other(market_type, tx, duration),
        _ => panic!("Unknown exchange {}", exchange),
    }
}
