#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_markets::MarketType;
use utils::parse;

const EXCHANGE_NAME: &str = "binance";

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_211231")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_211231")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_211231")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_211231")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_211231")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_211231")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_bbo(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_bbo,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::BBO
    )
}

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_211231")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_211231")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_l2_topk(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_topk,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2TopK
    )
}

#[test_case(MarketType::Spot, "BTCUSDT"; "inconclusive since spot market has too many symbols")]
#[test_case(MarketType::InverseFuture, "BTCUSD_211231")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_211231")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive since option market has too many symbols")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::Spot)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    test_all_symbols!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_211231")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_211231")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_ticker(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_ticker,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Ticker
    )
}

#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
fn test_crawl_funding_rate(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_funding_rate,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::FundingRate
    )
}

#[test_case(MarketType::Spot)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_crawl_candlestick(market_type: MarketType) {
    gen_test_crawl_candlestick!(EXCHANGE_NAME, market_type)
}
