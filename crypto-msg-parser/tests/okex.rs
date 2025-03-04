mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{extract_symbol, parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn spot() {
        let raw_msg = r#"{"table":"spot/trade","data":[{"side":"sell","trade_id":"161659503","price":"56593.6","size":"0.00020621","instrument_id":"BTC-USDT","timestamp":"2021-03-22T01:16:28.687Z"}]}"#;
        let trades = &parse_trade("okex", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okex",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("okex", MarketType::Spot, raw_msg).unwrap(),
            trade,
        );

        assert_eq!(trade.quantity_base, 0.00020621);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"table":"futures/trade","data":[{"side":"buy","trade_id":"5430565","price":"60059.7","qty":"20","instrument_id":"BTC-USDT-210625","timestamp":"2021-03-22T01:32:18.087Z"}]}"#;
        let trades = &parse_trade("okex", MarketType::LinearFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okex",
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            extract_symbol("okex", MarketType::LinearFuture, raw_msg).unwrap(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            20.0 * 0.01,
            epsilon = 0.00000001
        ));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            20.0 * 0.01 * 60059.7,
            epsilon = 0.001
        ));
        assert_eq!(trade.quantity_contract, Some(20.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"table":"swap/trade","data":[{"side":"buy","trade_id":"62257592","price":"56480.1","size":"3","instrument_id":"BTC-USDT-SWAP","timestamp":"2021-03-22T01:33:00.684Z"}]}"#;
        let trades = &parse_trade("okex", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okex",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol("okex", MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            0.01 * 3.0,
            epsilon = 0.000000001
        ));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            0.01 * 3.0 * 56480.1,
            epsilon = 0.0001
        ));
        assert_eq!(trade.quantity_contract, Some(3.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"table":"futures/trade","data":[{"side":"sell","trade_id":"16606935","price":"59999.7","qty":"7","instrument_id":"BTC-USD-210625","timestamp":"2021-03-22T01:32:41.377Z"}]}"#;
        let trades = &parse_trade("okex", MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okex",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("okex", MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
        );

        assert_eq!(trade.quantity_base, 100.0 * 7.0 / 59999.7);
        assert_eq!(trade.quantity_quote, 100.0 * 7.0);
        assert_eq!(trade.quantity_contract, Some(7.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"swap/trade","data":[{"side":"sell","trade_id":"102067670","price":"56535.9","size":"1","instrument_id":"BTC-USD-SWAP","timestamp":"2021-03-22T01:33:14.051Z"}]}"#;
        let trades = &parse_trade("okex", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okex",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("okex", MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
        );

        assert_eq!(trade.quantity_base, 100.0 * 1.0 / 56535.9);
        assert_eq!(trade.quantity_quote, 100.0 * 1.0);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"table":"option/trade","data":[{"side":"buy","trade_id":"231","price":"0.1545","qty":"4","instrument_id":"BTC-USD-210625-72000-C","timestamp":"2021-03-20T12:01:16.947Z"}]}"#;
        let trades = &parse_trade("okex", MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okex",
            MarketType::EuropeanOption,
            "BTC/USD".to_string(),
            extract_symbol("okex", MarketType::EuropeanOption, raw_msg).unwrap(),
            trade,
        );

        assert_eq!(trade.quantity_base, 0.1 * 4.0);
        assert_eq!(trade.quantity_quote, 0.1 * 4.0 * 0.1545);
        assert_eq!(trade.quantity_contract, Some(4.0));
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"{"table":"option/trades","data":[{"instrument_id":"BTC-USD-210924-120000-C","trade_id":"22","price":"0.079","qty":"1","trade_side":"sell","timestamp":"2021-03-23T08:12:28.348Z"}]}"#;
        let trades = &parse_trade("okex", MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okex",
            MarketType::EuropeanOption,
            "BTC/USD".to_string(),
            extract_symbol("okex", MarketType::EuropeanOption, raw_msg).unwrap(),
            trade,
        );

        assert_eq!(trade.quantity_base, 0.1 * 1.0);
        assert_eq!(trade.quantity_quote, 0.1 * 1.0 * 0.079);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_msg_parser::{parse_funding_rate, MarketType};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"swap/funding_rate","data":[{"estimated_rate":"0.00065","funding_rate":"0.00072933","funding_time":"2021-04-02T00:00:00.000Z","instrument_id":"BTC-USD-SWAP","interest_rate":"0","settlement_time":"2021-04-02T08:00:00.000Z"}]}"#;
        let funding_rates = &parse_funding_rate("okex", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("okex", MarketType::InverseSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00072933);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.00065));
        assert_eq!(funding_rates[0].funding_time, 1617321600000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"table":"swap/funding_rate","data":[{"estimated_rate":"0.00031","funding_rate":"0.00081859","funding_time":"2021-04-02T00:00:00.000Z","instrument_id":"BTC-USDT-SWAP","interest_rate":"0","settlement_time":"2021-04-02T08:00:00.000Z"}]}"#;
        let funding_rates = &parse_funding_rate("okex", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("okex", MarketType::LinearSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00081859);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.00031));
        assert_eq!(funding_rates[0].funding_time, 1617321600000);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_msg_parser::{extract_symbol, parse_l2, MarketType};

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"{"table":"spot/depth_l2_tbt","action":"partial","data":[{"instrument_id":"BTC-USDT","asks":[["38930","3.84264467","0","12"],["38932.4","0.00135697","0","3"],["38932.5","0.14401147","0","2"]],"bids":[["38929.9","0.05005381","0","4"],["38925.7","0.00062109","0","2"],["38925.6","0.21438503","0","1"]],"timestamp":"2021-06-03T12:39:11.253Z","checksum":1860980846}]}"#;
        let orderbook = &parse_l2("okex", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okex",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("okex", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622723951253);

        assert_eq!(orderbook.bids[0].price, 38929.9);
        assert_eq!(orderbook.bids[0].quantity_base, 0.05005381);
        assert_eq!(orderbook.bids[0].quantity_quote, 38929.9 * 0.05005381);

        assert_eq!(orderbook.asks[0].price, 38930.0);
        assert_eq!(orderbook.asks[0].quantity_base, 3.84264467);
        assert_eq!(orderbook.asks[0].quantity_quote, 38930.0 * 3.84264467);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"table":"spot/depth_l2_tbt","action":"update","data":[{"instrument_id":"BTC-USDT","asks":[["38888.7","4.14263198","0","12"]],"bids":[["38886.3","0","0","0"]],"timestamp":"2021-06-03T12:40:09.962Z","checksum":976527820}]}"#;
        let orderbook = &parse_l2("okex", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okex",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("okex", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622724009962);

        assert_eq!(orderbook.bids[0].price, 38886.3);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);

        assert_eq!(orderbook.asks[0].price, 38888.7);
        assert_eq!(orderbook.asks[0].quantity_base, 4.14263198);
        assert_eq!(orderbook.asks[0].quantity_quote, 38888.7 * 4.14263198);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"table":"futures/depth_l2_tbt","action":"partial","data":[{"instrument_id":"BTC-USDT-210625","asks":[["39302.5","1","0","1"],["39302.6","5","0","2"],["39304.3","21","0","1"]],"bids":[["39302.2","4","0","1"],["39300.7","5","0","1"],["39299","4","0","1"]],"timestamp":"2021-06-03T13:09:34.429Z","checksum":698961978}]}"#;
        let orderbook = &parse_l2("okex", MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okex",
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            extract_symbol("okex", MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622725774429);

        assert_eq!(orderbook.asks[0].price, 39302.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.01);
        assert_eq!(orderbook.asks[0].quantity_quote, 39302.5 * 0.01);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1.0);

        assert_eq!(orderbook.bids[0].price, 39302.2);
        assert_eq!(orderbook.bids[0].quantity_base, 0.04);
        assert_eq!(orderbook.bids[0].quantity_quote, 39302.2 * 0.04);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 4.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"table":"swap/depth_l2_tbt","action":"partial","data":[{"instrument_id":"BTC-USD-SWAP","asks":[["39167.2","130","0","3"],["39169.6","45","0","1"],["39173.1","1","0","1"]],"bids":[["39167.1","1536","0","8"],["39166.2","68","0","1"],["39165.9","47","0","1"]],"timestamp":"2021-06-03T13:14:24.831Z","checksum":-1582320415}]}"#;
        let orderbook = &parse_l2("okex", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okex",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("okex", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622726064831);

        assert_eq!(orderbook.asks[0].price, 39167.2);
        assert_eq!(orderbook.asks[0].quantity_base, 13000.0 / 39167.2);
        assert_eq!(orderbook.asks[0].quantity_quote, 13000.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 130.0);

        assert_eq!(orderbook.bids[0].price, 39167.1);
        assert_eq!(orderbook.bids[0].quantity_base, 153600.0 / 39167.1);
        assert_eq!(orderbook.bids[0].quantity_quote, 153600.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1536.0);
    }

    #[test]
    fn option_snapshot() {
        let raw_msg = r#"{"table":"option/depth_l2_tbt","action":"partial","data":[{"instrument_id":"BTC-USD-210604-30000-P","asks":[["0.0015","906","0","3"]],"bids":[],"timestamp":"2021-06-03T13:18:55.745Z","checksum":-288111842}]}"#;
        let orderbook = &parse_l2("okex", MarketType::EuropeanOption, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okex",
            MarketType::EuropeanOption,
            "BTC/USD".to_string(),
            extract_symbol("okex", MarketType::EuropeanOption, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622726335745);

        assert_eq!(orderbook.asks[0].price, 0.0015);
        assert_eq!(orderbook.asks[0].quantity_base, 0.1 * 906.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.1 * 906.0 * 0.0015);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 906.0);
    }
}
