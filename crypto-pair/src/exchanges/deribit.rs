pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.ends_with("-PERPETUAL") {
        // inverse_swap
        let base = symbol.strip_suffix("-PERPETUAL").unwrap();
        Some(format!("{}/USD", base))
    } else if symbol.len() > 7 && (&symbol[(symbol.len() - 2)..]).parse::<i64>().is_ok() {
        // inverse_future
        let pos = symbol.find('-').unwrap();
        let base = &symbol[..pos];
        Some(format!("{}/USD", base))
    } else if symbol.ends_with("-P") || symbol.ends_with("-C") {
        // option
        let pos = symbol.find('-').unwrap();
        let base = &symbol[..pos];
        Some(format!("{}/{}", base, base))
    } else {
        None
    }
}
