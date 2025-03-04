use crypto_market_type::MarketType;

use crate::{
    exchanges::{kucoin::message::WebsocketMsg, utils::calc_quantity_and_volume},
    MessageType, Order, OrderBookMsg, TradeMsg, TradeSide,
};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "kucoin";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    baseCurrency: String,
    multiplier: f64,
    isInverse: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ResponseMsg {
    code: String,
    data: Vec<SwapMarket>,
}

// https://docs.kucoin.cc/futures/#execution-data
#[derive(Serialize, Deserialize)]
struct ContractTradeMsg {
    symbol: String,
    sequence: i64,
    side: String, // buy, sell
    size: f64,
    price: f64,
    ts: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.kucoin.cc/futures/#level-2-market-data
#[derive(Serialize, Deserialize)]
struct ContractOrderbookMsg {
    sequence: i64,
    change: String, // Price, side, quantity
    timestamp: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<ContractTradeMsg>>(msg)?;
    debug_assert_eq!(ws_msg.subject, "match");
    debug_assert!(ws_msg.topic.starts_with("/contractMarket/execution:"));
    let raw_trade = ws_msg.data;
    let pair = crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap();
    let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        raw_trade.price,
        raw_trade.size,
    );

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.symbol.clone(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.ts / 1000000,
        price: raw_trade.price,
        quantity_base,
        quantity_quote,
        quantity_contract,
        side: if raw_trade.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.sequence.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<ContractOrderbookMsg>>(msg)?;
    debug_assert_eq!(ws_msg.subject, "level2");
    debug_assert!(ws_msg.topic.starts_with("/contractMarket/level2:"));
    let symbol = ws_msg
        .topic
        .strip_prefix("/contractMarket/level2:")
        .unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let raw_order: Vec<&str> = ws_msg.data.change.split(',').collect();
    let order: Order = {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity = raw_order[2].parse::<f64>().unwrap();

        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let mut asks: Vec<Order> = Vec::new();
    let mut bids: Vec<Order> = Vec::new();
    if raw_order[1] == "sell" {
        asks.push(order);
    } else {
        bids.push(order);
    }

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp: ws_msg.data.timestamp,
        seq_id: Some(ws_msg.data.sequence as u64),
        prev_seq_id: None,
        asks,
        bids,
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
