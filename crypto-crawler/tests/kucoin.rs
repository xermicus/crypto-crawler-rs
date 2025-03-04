#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_markets::MarketType;
use utils::parse;

const EXCHANGE_NAME: &str = "kucoin";

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21"; "inconclusive")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21")]
fn test_crawl_bbo(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_bbo,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::BBO
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21")]
fn test_crawl_l2_topk(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_topk,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2TopK
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21")]
fn test_crawl_l3_event(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l3_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L3Event
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21")]
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
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::InverseFuture)]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    test_all_symbols!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::Spot, "BTC-USDT"; "inconclusive")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21")]
fn test_crawl_l3_snapshot(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l3_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L3Snapshot
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
#[test_case(MarketType::InverseFuture, "XBTMZ21")]
fn test_crawl_ticker(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_ticker,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Ticker
    )
}

#[test_case(MarketType::Spot)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::InverseFuture)]
fn test_crawl_candlestick(market_type: MarketType) {
    gen_test_crawl_candlestick!(EXCHANGE_NAME, market_type)
}
