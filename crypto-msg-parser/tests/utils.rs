use crypto_market_type::MarketType;
use crypto_msg_parser::{FundingRateMsg, MessageType, OrderBookMsg, TradeMsg};
use float_cmp::approx_eq;

pub fn check_trade_fields(
    exchange: &str,
    market_type: MarketType,
    pair: String,
    symbol: String,
    trade: &TradeMsg,
) {
    assert_eq!(trade.exchange, exchange);
    assert_eq!(trade.market_type, market_type);
    assert_eq!(trade.pair, pair);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.msg_type, MessageType::Trade);
    assert!(trade.price > 0.0);
    assert!(trade.quantity_base > 0.0);
    assert!(trade.quantity_quote > 0.0);
    if exchange != "bitmex" {
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            trade.price * trade.quantity_base,
            epsilon = 0.0000000001
        ));
    }
    assert!(!trade.trade_id.is_empty());
    assert_eq!(trade.timestamp.to_string().len(), 13);
}

// TODO: weird, it is actually being used
#[allow(dead_code)]
pub fn check_orderbook_fields(
    exchange: &str,
    market_type: MarketType,
    pair: String,
    symbol: String,
    orderbook: &OrderBookMsg,
) {
    assert_eq!(orderbook.exchange, exchange);
    assert_eq!(orderbook.market_type, market_type);
    assert_eq!(orderbook.pair, pair);
    assert_eq!(orderbook.symbol, symbol);
    assert_eq!(orderbook.msg_type, MessageType::L2Event);
    assert_eq!(orderbook.timestamp.to_string().len(), 13);

    for order in orderbook.asks.iter() {
        assert!(order.price > 0.0);
        assert!(order.quantity_base >= 0.0);
        assert!(order.quantity_quote >= 0.0);

        if let Some(quantity_contract) = order.quantity_contract {
            assert!(quantity_contract >= 0.0);
        }
    }
}

// TODO: fake warning
#[allow(dead_code)]
pub fn check_funding_rate_fields(
    exchange: &str,
    market_type: MarketType,
    funding_rate: &FundingRateMsg,
) {
    assert_eq!(funding_rate.exchange, exchange);
    assert_eq!(funding_rate.market_type, market_type);
    // assert_eq!(funding_rate.pair, pair);
    assert_eq!(funding_rate.msg_type, MessageType::FundingRate);
    assert!(funding_rate.funding_rate > -1.0);
    assert!(funding_rate.funding_rate < 1.0);
    if exchange == "bitmex" {
        assert_eq!(funding_rate.funding_time % (4 * 3600000), 0);
    } else if exchange == "bitget" {
        assert_eq!(funding_rate.funding_time % 3600000, 0);
    } else {
        assert_eq!(funding_rate.funding_time % (8 * 3600000), 0);
    }
}
