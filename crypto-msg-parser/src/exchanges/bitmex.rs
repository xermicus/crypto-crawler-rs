use crypto_market_type::MarketType;

use crate::exchanges::utils::{calc_quantity_and_volume, http_get};
use crate::Order;
use crate::{FundingRateMsg, MessageType, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use chrono::DateTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::{BTreeMap, HashMap};

const EXCHANGE_NAME: &str = "bitmex";

lazy_static! {
    // symbol -> tickSize
    static ref SYMBOL_INDEX_AND_TICK_SIZE_MAP: HashMap<String, (usize, f64)> = {
        let mut m: HashMap<String, (usize, f64)> = vec![
            ("A50G16", (65, 2.5)),
            ("A50H16", (67, 2.5)),
            ("A50J16", (72, 2.5)),
            ("A50K16", (73, 2.5)),
            ("A50M16", (99, 2.5)),
            ("A50N16", (120, 2.5)),
            ("A50Q16", (124, 2.5)),
            ("AAVEUSDT", (589, 0.01)),
            ("ADAF18", (260, 0.00000001)),
            ("ADAH18", (267, 0.00000001)),
            ("ADAH19", (323, 0.00000001)),
            ("ADAH20", (366, 0.00000001)),
            ("ADAH21", (459, 0.00000001)),
            ("ADAM18", (271, 0.00000001)),
            ("ADAM19", (331, 0.00000001)),
            ("ADAM20", (383, 0.00000001)),
            ("ADAM21", (523, 0.00000001)),
            ("ADAU18", (286, 0.00000001)),
            ("ADAU19", (339, 0.00000001)),
            ("ADAU20", (398, 0.00000001)),
            ("ADAU21", (570, 0.00000001)),
            ("ADAUSD", (676, 0.0001)),
            ("ADAUSDT", (521, 0.00001)),
            ("ADAUSDTH21", (463, 0.00001)),
            ("ADAUSDTZ20", (431, 0.00001)),
            ("ADAZ18", (303, 0.00000001)),
            ("ADAZ19", (349, 0.00000001)),
            ("ADAZ20", (427, 0.00000001)),
            ("ADAZ21", (638, 0.00000001)),
            ("ALTMEXUSD", (616, 0.01)),
            ("AVAXUSD", (675, 0.001)),
            ("AXSUSD", (706, 0.01)),
            ("AXSUSDT", (625, 0.001)),
            ("BCHF18", (256, 0.0001)),
            ("BCHH18", (266, 0.0001)),
            ("BCHH19", (322, 0.0001)),
            ("BCHH20", (365, 0.00001)),
            ("BCHH21", (458, 0.00001)),
            ("BCHM18", (272, 0.0001)),
            ("BCHM19", (330, 0.00001)),
            ("BCHM20", (382, 0.00001)),
            ("BCHM21", (524, 0.00001)),
            ("BCHU18", (285, 0.0001)),
            ("BCHU19", (338, 0.00001)),
            ("BCHU20", (397, 0.00001)),
            ("BCHU21", (569, 0.000001)),
            ("BCHUSD", (402, 0.05)),
            ("BCHX17", (239, 0.0001)),
            ("BCHZ17", (240, 0.0001)),
            ("BCHZ18", (302, 0.0001)),
            ("BCHZ19", (348, 0.00001)),
            ("BCHZ20", (426, 0.00001)),
            ("BCHZ21", (637, 0.000001)),
            ("BFXQ16", (128, 0.0001)),
            ("BFXU16", (132, 0.0001)),
            ("BFXV16", (136, 0.0001)),
            ("BNBUSD", (678, 0.01)),
            ("BNBUSDT", (542, 0.01)),
            ("BNBUSDTH21", (467, 0.0005)),
            ("BNBUSDTZ20", (451, 0.0005)),
            ("BVOL24H", (34, 0.01)),
            ("BVOL7D", (36, 0.01)),
            ("BVOLG15", (18, 0.01)),
            ("BVOLH15", (23, 0.01)),
            ("BVOLJ15", (25, 0.01)),
            ("B_BLOCKSZ17", (180, 0.01)),
            ("B_SEGWITZ17", (181, 0.01)),
            ("COIN_BH17", (164, 0.01)),
            ("DAOETH", (94, 0.00001)),
            ("DASH7D", (165, 0.000001)),
            ("DASHH18", (252, 0.000001)),
            ("DASHJ17", (178, 0.000001)),
            ("DASHM17", (186, 0.000001)),
            ("DASHU17", (209, 0.000001)),
            ("DASHZ17", (225, 0.000001)),
            ("DEFIMEXUSD", (617, 0.01)),
            ("DOGEUSD", (677, 0.00001)),
            ("DOGEUSDT", (476, 0.00001)),
            ("DOTUSD", (679, 0.001)),
            ("DOTUSDT", (519, 0.0005)),
            ("DOTUSDTH21", (469, 0.0005)),
            ("DOTUSDTZ20", (453, 0.0005)),
            ("EOSH19", (324, 0.0000001)),
            ("EOSH20", (367, 0.0000001)),
            ("EOSH21", (460, 0.0000001)),
            ("EOSM18", (279, 0.0000001)),
            ("EOSM19", (332, 0.0000001)),
            ("EOSM20", (384, 0.0000001)),
            ("EOSM21", (525, 0.0000001)),
            ("EOSN17", (216, 0.000001)),
            ("EOSU18", (287, 0.0000001)),
            ("EOSU19", (340, 0.0000001)),
            ("EOSU20", (399, 0.0000001)),
            ("EOSU21", (571, 0.00000001)),
            ("EOSUSD", (707, 0.0001)),
            ("EOSUSDT", (539, 0.0005)),
            ("EOSUSDTH21", (464, 0.0005)),
            ("EOSUSDTZ20", (432, 0.0005)),
            ("EOSZ18", (304, 0.0000001)),
            ("EOSZ19", (350, 0.0000001)),
            ("EOSZ20", (428, 0.0000001)),
            ("EOSZ21", (639, 0.00000001)),
            ("ETC24H", (123, 0.000001)),
            ("ETC7D", (125, 0.000001)),
            ("ETH7D", (54, 0.00001)),
            ("ETHH18", (251, 0.00001)),
            ("ETHH19", (319, 0.00001)),
            ("ETHH20", (362, 0.00001)),
            ("ETHH21", (455, 0.00001)),
            ("ETHJ17", (177, 0.00001)),
            ("ETHM17", (185, 0.00001)),
            ("ETHM18", (273, 0.00001)),
            ("ETHM19", (327, 0.00001)),
            ("ETHM20", (379, 0.00001)),
            ("ETHM21", (526, 0.00001)),
            ("ETHU17", (208, 0.00001)),
            ("ETHU18", (282, 0.00001)),
            ("ETHU19", (335, 0.00001)),
            ("ETHU20", (394, 0.00001)),
            ("ETHU21", (566, 0.00001)),
            ("ETHUSD", (297, 0.05)),
            ("ETHUSDH21", (462, 0.05)),
            ("ETHUSDM20", (386, 0.05)),
            ("ETHUSDM21", (527, 0.05)),
            ("ETHUSDU20", (401, 0.05)),
            ("ETHUSDU21", (573, 0.05)),
            ("ETHUSDZ20", (430, 0.05)),
            ("ETHUSDZ21", (641, 0.05)),
            ("ETHXBT", (78, 0.00001)),
            ("ETHZ17", (224, 0.00001)),
            ("ETHZ18", (299, 0.00001)),
            ("ETHZ19", (345, 0.00001)),
            ("ETHZ20", (423, 0.00001)),
            ("ETHZ21", (634, 0.00001)),
            ("FCT7D", (70, 0.000001)),
            ("FCTM17", (190, 0.000001)),
            ("FCTXBT", (93, 0.000001)),
            ("FILUSDT", (556, 0.01)),
            ("GNOM17", (184, 0.000001)),
            ("LINKUSD", (708, 0.001)),
            ("LINKUSDT", (441, 0.0005)),
            ("LINKUSDTH21", (465, 0.0005)),
            ("LINKUSDTM21", (528, 0.0005)),
            ("LINKUSDTZ20", (433, 0.0005)),
            ("LSKXBT", (98, 0.000001)),
            ("LTC7D", (150, 0.00001)),
            ("LTCH18", (254, 0.00001)),
            ("LTCH19", (320, 0.00001)),
            ("LTCH20", (363, 0.000005)),
            ("LTCH21", (456, 0.000005)),
            ("LTCM17", (188, 0.00001)),
            ("LTCM18", (274, 0.00001)),
            ("LTCM19", (328, 0.000005)),
            ("LTCM20", (380, 0.000005)),
            ("LTCM21", (529, 0.000005)),
            ("LTCU17", (211, 0.00001)),
            ("LTCU18", (283, 0.00001)),
            ("LTCU19", (336, 0.000005)),
            ("LTCU20", (395, 0.000005)),
            ("LTCU21", (567, 0.000001)),
            ("LTCUSD", (407, 0.01)),
            ("LTCXBT", (85, 0.00001)),
            ("LTCZ17", (227, 0.00001)),
            ("LTCZ18", (300, 0.00001)),
            ("LTCZ19", (346, 0.000005)),
            ("LTCZ20", (424, 0.000005)),
            ("LTCZ21", (635, 0.000001)),
            ("LUNAUSD", (649, 0.001)),
            ("MATICUSDT", (588, 0.0001)),
            ("NEOG18", (269, 0.000001)),
            ("NEOH18", (270, 0.000001)),
            ("QTUMU17", (195, 0.000001)),
            ("REP7D", (144, 0.000001)),
            ("SNTN17", (202, 0.00000001)),
            ("SOLUSD", (709, 0.01)),
            ("SOLUSDT", (549, 0.001)),
            ("SRMUSDT", (632, 0.001)),
            ("SUSHIUSDT", (618, 0.001)),
            ("TRXH19", (325, 0.00000001)),
            ("TRXH20", (368, 0.00000001)),
            ("TRXH21", (461, 0.00000001)),
            ("TRXM19", (333, 0.00000001)),
            ("TRXM20", (385, 0.00000001)),
            ("TRXM21", (530, 0.00000001)),
            ("TRXU18", (290, 0.00000001)),
            ("TRXU19", (341, 0.00000001)),
            ("TRXU20", (400, 0.00000001)),
            ("TRXU21", (572, 0.0000000001)),
            ("TRXUSDT", (540, 0.00001)),
            ("TRXZ18", (305, 0.00000001)),
            ("TRXZ19", (351, 0.00000001)),
            ("TRXZ20", (429, 0.00000001)),
            ("TRXZ21", (640, 0.0000000001)),
            ("UNIUSDT", (520, 0.001)),
            ("VETUSDT", (581, 0.00001)),
            ("WINZ16", (156, 0.000001)),
            ("XBCH17", (158, 0.1)),
            ("XBCM17", (174, 0.1)),
            ("XBCZ16", (155, 0.1)),
            ("XBJ24H", (106, 1.0)),
            ("XBJ7D", (137, 1.0)),
            ("XBJH17", (167, 1.0)),
            ("XBJM17", (175, 1.0)),
            ("XBJU17", (206, 1.0)),
            ("XBJZ16", (138, 1.0)),
            ("XBJZ17", (230, 100.0)),
            ("XBT24H", (58, 0.01)),
            ("XBT48H", (66, 0.01)),
            ("XBT7D", (51, 0.01)),
            ("XBT7D_D90", (278, 0.00001)),
            ("XBT7D_D95", (281, 0.00001)),
            ("XBT7D_U105", (280, 0.00001)),
            ("XBT7D_U110", (277, 0.00001)),
            ("XBTEUR", (564, 0.5)),
            ("XBTEURU21", (574, 0.5)),
            ("XBTEURZ21", (642, 0.5)),
            ("XBTF15", (1, 0.01)),
            ("XBTF15_G15", (13, 0.01)),
            ("XBTF15_H15", (4, 0.01)),
            ("XBTG15", (12, 0.01)),
            ("XBTH15", (3, 0.01)),
            ("XBTH15_G15", (14, 0.01)),
            ("XBTH16", (55, 0.01)),
            ("XBTH17", (157, 0.01)),
            ("XBTH18", (249, 0.5)),
            ("XBTH19", (298, 0.5)),
            ("XBTH20", (344, 0.5)),
            ("XBTH21", (422, 0.5)),
            ("XBTH22", (633, 0.5)),
            ("XBTJ15", (26, 0.01)),
            ("XBTK15", (27, 0.01)),
            ("XBTK15_M15", (30, 0.01)),
            ("XBTM15", (29, 0.01)),
            ("XBTM15_U15", (40, 0.01)),
            ("XBTM15_Z15", (42, 0.01)),
            ("XBTM16", (62, 0.01)),
            ("XBTM17", (173, 0.1)),
            ("XBTM18", (259, 0.5)),
            ("XBTM19", (318, 0.5)),
            ("XBTM20", (361, 0.5)),
            ("XBTM21", (454, 0.5)),
            ("XBTN15", (44, 0.01)),
            ("XBTQ15", (46, 0.01)),
            ("XBTU15", (39, 0.01)),
            ("XBTU15_Z15", (43, 0.01)),
            ("XBTU16", (71, 0.01)),
            ("XBTU17", (205, 0.1)),
            ("XBTU18", (276, 0.5)),
            ("XBTU19", (326, 0.5)),
            ("XBTU20", (378, 0.5)),
            ("XBTU21", (532, 0.5)),
            ("XBTUSD", (88, 0.01)),
            ("XBTV15", (56, 0.01)),
            ("XBTV21", (650, 0.5)),
            ("XBTX21", (705, 0.5)),
            ("XBTZ14", (0, 0.01)),
            ("XBTZ14_F15", (2, 0.01)),
            ("XBTZ14_H15", (11, 0.01)),
            ("XBTZ15", (41, 0.01)),
            ("XBTZ16", (149, 0.01)),
            ("XBTZ17", (229, 0.5)),
            ("XBTZ18", (291, 0.5)),
            ("XBTZ19", (334, 0.5)),
            ("XBTZ20", (393, 0.5)),
            ("XBTZ21", (565, 0.5)),
            ("XBU24H", (22, 0.01)),
            ("XBU7D", (63, 0.01)),
            ("XBUH15", (6, 0.01)),
            ("XBUH15_M15", (10, 0.01)),
            ("XBUH15_U15", (16, 0.01)),
            ("XBUJ15", (24, 0.01)),
            ("XBUK15", (28, 0.01)),
            ("XBUM15", (8, 0.01)),
            ("XBUM15_U15", (17, 0.01)),
            ("XBUN15", (45, 0.01)),
            ("XBUQ15", (47, 0.01)),
            ("XBUU15", (15, 0.01)),
            ("XBUU15_Z15", (38, 0.01)),
            ("XBUV15", (57, 0.01)),
            ("XBUZ14", (5, 0.01)),
            ("XBUZ14_H15", (7, 0.01)),
            ("XBUZ14_M15", (9, 0.01)),
            ("XBUZ15", (37, 0.01)),
            ("XLMF18", (265, 0.00000001)),
            ("XLMH18", (268, 0.00000001)),
            ("XLMUSDT", (522, 0.00001)),
            ("XLT7D", (50, 0.001)),
            ("XMR7D", (131, 0.000001)),
            ("XMRH18", (253, 0.000001)),
            ("XMRJ17", (179, 0.000001)),
            ("XMRM17", (187, 0.000001)),
            ("XMRU17", (210, 0.000001)),
            ("XMRZ17", (226, 0.000001)),
            ("XRP7D", (143, 0.00000001)),
            ("XRPH18", (255, 0.00000001)),
            ("XRPH19", (321, 0.00000001)),
            ("XRPH20", (364, 0.00000001)),
            ("XRPH21", (457, 0.00000001)),
            ("XRPM17", (189, 0.00000001)),
            ("XRPM18", (275, 0.00000001)),
            ("XRPM19", (329, 0.00000001)),
            ("XRPM20", (381, 0.00000001)),
            ("XRPM21", (531, 0.00000001)),
            ("XRPU17", (212, 0.00000001)),
            ("XRPU18", (284, 0.00000001)),
            ("XRPU19", (337, 0.00000001)),
            ("XRPU20", (396, 0.00000001)),
            ("XRPU21", (568, 0.00000001)),
            ("XRPUSD", (377, 0.0001)),
            ("XRPZ17", (228, 0.00000001)),
            ("XRPZ18", (301, 0.00000001)),
            ("XRPZ19", (347, 0.00000001)),
            ("XRPZ20", (425, 0.00000001)),
            ("XRPZ21", (636, 0.00000001)),
            ("XTZUSDTH21", (466, 0.0005)),
            ("XTZUSDTZ20", (434, 0.0005)),
            ("XTZZ17", (215, 0.000001)),
            ("YFIUSDTH21", (468, 0.5)),
            ("YFIUSDTZ20", (452, 0.5)),
            ("ZECH17", (159, 0.000001)),
            ("ZECH18", (250, 0.000001)),
            ("ZECM17", (176, 0.000001)),
            ("ZECU17", (207, 0.000001)),
            ("ZECZ16", (135, 0.000001)),
            ("ZECZ17", (223, 0.000001)),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_tick_sizes();
        for (symbol, tick_size) in from_online {
            m.insert(symbol, tick_size);
        }

        m
    };
}

fn fetch_tick_sizes() -> BTreeMap<String, (usize, f64)> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct TickSize {
        symbol: String,
        timestamp: String,
        tickSize: f64,
    }
    let mut m: BTreeMap<String, (usize, f64)> = BTreeMap::new();
    let mut start = 0_usize;
    loop {
        let url = format!(
            "https://www.bitmex.com/api/v1/instrument?columns=symbol,tickSize&start={}&count=500",
            start
        );
        if let Ok(txt) = http_get(url.as_str()) {
            if let Ok(tick_sizes) = serde_json::from_str::<Vec<TickSize>>(&txt) {
                let n = tick_sizes.len();
                for (index, tick_size) in tick_sizes.into_iter().enumerate() {
                    let real_tick_size = if tick_size.symbol == "XBTUSD" {
                        0.01 // legacy reason, see https://www.bitmex.com/app/wsAPI#OrderBookL2
                    } else {
                        tick_size.tickSize
                    };
                    if !tick_size.symbol.starts_with('.') {
                        m.insert(tick_size.symbol, (start + index, real_tick_size));
                    }
                }
                if n < 500 {
                    break;
                } else {
                    start += 500;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    m
}

// see https://www.bitmex.com/app/wsAPI#Response-Format
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    timestamp: String,
    symbol: String,
    side: String, // Sell, Buy'
    size: f64,
    price: f64,
    tickDirection: String, // MinusTick, PlusTick, ZeroMinusTick, ZeroPlusTick
    trdMatchID: String,
    grossValue: f64,
    homeNotional: f64,
    foreignNotional: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawOrder {
    symbol: String,
    id: usize,
    side: String, // Sell, Buy
    size: Option<f64>,
    price: Option<f64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawFundingRateMsg {
    timestamp: String,
    symbol: String,
    fundingInterval: String,
    fundingRate: f64,
    fundingRateDaily: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    table: String,
    action: String,
    data: Vec<T>,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    let symbols = ws_msg
        .data
        .iter()
        .map(|v| v["symbol"].as_str().unwrap())
        .collect::<Vec<&str>>();
    if symbols.is_empty() {
        None
    } else {
        Some(symbols[0].to_string())
    }
}

// Copied from crypto-markets/tests/bitmex.rs
fn get_market_type_from_symbol(symbol: &str) -> MarketType {
    let date = &symbol[(symbol.len() - 2)..];
    if date.parse::<i64>().is_ok() {
        // future
        if symbol.starts_with("XBT") {
            // Settled in XBT, quoted in USD
            MarketType::InverseFuture
        } else if (&symbol[..(symbol.len() - 3)]).ends_with("USD") {
            // Settled in XBT, quoted in USD
            MarketType::QuantoFuture
        } else {
            // Settled in XBT, quoted in XBT
            MarketType::LinearFuture
        }
    } else {
        // swap
        if symbol.starts_with("XBT") {
            // Settled in XBT, quoted in USD
            MarketType::InverseSwap
        } else {
            MarketType::QuantoSwap
        }
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg)?;
    let raw_trades = ws_msg.data;
    let mut trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            // assert_eq!(raw_trade.foreignNotional, raw_trade.homeNotional * raw_trade.price); // tiny diff actually exists
            let timestamp = DateTime::parse_from_rfc3339(&raw_trade.timestamp).unwrap();
            let market_type = if market_type == MarketType::Unknown {
                get_market_type_from_symbol(&raw_trade.symbol)
            } else {
                market_type
            };
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.symbol.to_string(),
                pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp: timestamp.timestamp_millis(),
                price: raw_trade.price,
                quantity_base: raw_trade.homeNotional,
                quantity_quote: raw_trade.foreignNotional,
                quantity_contract: Some(raw_trade.size),
                side: if raw_trade.side == "Sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.trdMatchID.clone(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();
    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawFundingRateMsg>>(msg)?;
    let mut rates: Vec<FundingRateMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_msg| {
            let settlement_time = DateTime::parse_from_rfc3339(&raw_msg.timestamp).unwrap();
            let market_type = if market_type == MarketType::Unknown {
                get_market_type_from_symbol(&raw_msg.symbol)
            } else {
                market_type
            };
            FundingRateMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_msg.symbol.clone(),
                pair: crypto_pair::normalize_pair(&raw_msg.symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::FundingRate,
                timestamp: Utc::now().timestamp_millis(),
                funding_rate: raw_msg.fundingRate,
                funding_time: settlement_time.timestamp_millis(),
                estimated_rate: None,
                json: serde_json::to_string(&raw_msg).unwrap(),
            }
        })
        .collect();
    if rates.len() == 1 {
        rates[0].json = msg.to_string();
    }
    Ok(rates)
}

/// convert ID to price
/// https://www.bitmex.com/app/wsAPI#OrderBookL2
/// price = (100000000 * symbolIdx - ID) * tickSize
pub fn id_to_price(symbol: &str, id: usize) -> f64 {
    let (index, tick_size) = SYMBOL_INDEX_AND_TICK_SIZE_MAP.get(symbol).unwrap();
    let (index, tick_size) = (*index, *tick_size);
    (100000000.0 * index as f64 - id as f64) * tick_size
}

/// convert price to ID
/// https://www.bitmex.com/app/wsAPI#OrderBookL2
/// ID = (100000000 * symbolIdx) - (price / tickSize)
pub fn price_to_id(symbol: &str, price: f64) -> usize {
    let (index, tick_size) = SYMBOL_INDEX_AND_TICK_SIZE_MAP.get(symbol).unwrap();
    let (index, tick_size) = (*index, *tick_size);

    (100000000.0 * index as f64 - price / tick_size) as usize
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: i64,
) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrder>>(msg)?;
    let snapshot = ws_msg.action == "partial";
    if ws_msg.data.is_empty() {
        return Ok(Vec::new());
    }
    let symbol = ws_msg.data[0].symbol.clone();
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    let market_type = if market_type == MarketType::Unknown {
        get_market_type_from_symbol(&symbol)
    } else {
        market_type
    };

    let parse_order = |raw_order: &RawOrder| -> Order {
        let price = if let Some(p) = raw_order.price {
            p
        } else {
            id_to_price(&raw_order.symbol, raw_order.id)
        };

        let quantity = raw_order.size.unwrap_or(0.0); // 0.0 means delete
        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair: pair.clone(),
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: None,
        prev_seq_id: None,
        asks: ws_msg
            .data
            .iter()
            .filter(|x| x.side == "Sell")
            .map(|x| parse_order(x))
            .collect(),
        bids: ws_msg
            .data
            .iter()
            .filter(|x| x.side == "Buy")
            .map(|x| parse_order(x))
            .collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_fetch_tick_sizes() {
        let tick_sizes = super::fetch_tick_sizes();
        assert!(tick_sizes.len() > 0);
        for (symbol, tick_size) in tick_sizes {
            println!("(\"{}\", ({}, {})),", symbol, tick_size.0, tick_size.1);
        }
    }

    #[test]
    fn test_id_to_price() {
        // data are from https://www.bitmex.com/api/v1/orderBook/L2?symbol=XBTUSD&depth=25
        assert_eq!(51366.5, super::id_to_price("XBTUSD", 8794863350));
        assert_eq!(51306.0, super::id_to_price("XBTUSD", 8794869400));

        assert_eq!(3460.0, super::id_to_price("ETHUSD", 29699930800));
        assert_eq!(3451.0, super::id_to_price("ETHUSD", 29699930980));

        assert_eq!(0.07369, super::id_to_price("ETHZ21", 63399992631));
        assert_eq!(0.07216, super::id_to_price("ETHZ21", 63399992784));
    }

    #[test]
    fn test_price_to_id() {
        assert_eq!(8794863350, super::price_to_id("XBTUSD", 51366.5));
        assert_eq!(8794869400, super::price_to_id("XBTUSD", 51306.0));

        assert_eq!(29699930800, super::price_to_id("ETHUSD", 3460.0));
        assert_eq!(29699930980, super::price_to_id("ETHUSD", 3451.0));

        assert_eq!(63399992631, super::price_to_id("ETHZ21", 0.07369));
        assert_eq!(63399992784, super::price_to_id("ETHZ21", 0.07216));
    }
}
