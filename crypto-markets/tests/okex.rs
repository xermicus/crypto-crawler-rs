use crypto_markets::{fetch_markets, fetch_symbols, get_market_types, MarketType};
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "okex";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains('-'));
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        assert!(symbol.contains("-USD-"));
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        assert!(symbol.contains("-USDT-"));
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USD-SWAP"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USDT-SWAP"));
    }
}

#[test]
fn fetch_option_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-C") || symbol.ends_with("-P"));
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "BTC-USDT")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.precision.tick_size, 0.1);
    assert_eq!(btc_usdt.precision.lot_size, 0.00000001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min, 0.00001);
    assert_eq!(quantity_limit.max, None);
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btc_usd = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC-USD-"))
        .unwrap();
    assert_eq!(btc_usd.precision.tick_size, 0.1);
    assert_eq!(btc_usd.precision.lot_size, 1.0);
    assert!(btc_usd.quantity_limit.is_none());
}

#[test]
fn fetch_linear_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC-USDT-"))
        .unwrap();
    assert_eq!(btc_usdt.precision.tick_size, 0.1);
    assert_eq!(btc_usdt.precision.lot_size, 1.0);
    assert!(btc_usdt.quantity_limit.is_none());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usd = markets.iter().find(|m| m.symbol == "BTC-USD-SWAP").unwrap();
    assert_eq!(btc_usd.precision.tick_size, 0.1);
    assert_eq!(btc_usd.precision.lot_size, 1.0);
    assert!(btc_usd.quantity_limit.is_none());
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "BTC-USDT-SWAP")
        .unwrap();
    assert_eq!(btc_usdt.precision.tick_size, 0.1);
    assert_eq!(btc_usdt.precision.lot_size, 1.0);
    assert!(btc_usdt.quantity_limit.is_none());
}

#[test]
fn fetch_option_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!markets.is_empty());

    let btc_usd = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC-USD-"))
        .unwrap();
    assert_eq!(btc_usd.precision.tick_size, 0.0005);
    assert_eq!(btc_usd.precision.lot_size, 1.0);
    assert!(btc_usd.quantity_limit.is_none());
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::EuropeanOption)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
