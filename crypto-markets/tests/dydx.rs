use crypto_markets::{fetch_markets, fetch_symbols, get_market_types, MarketType};
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "dydx";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USD"));
    }
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "BTC-USD")
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 1.0);
    assert_eq!(btcusd.precision.lot_size, 0.0001);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min, 0.001);
    assert_eq!(quantity_limit.max, None);
}

#[test_case(MarketType::LinearSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
