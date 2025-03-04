pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.ends_with("-PERP") {
        // linear swap
        let base = symbol.strip_suffix("-PERP").unwrap();
        Some(format!("{}/USD", base))
    } else if symbol.contains("-MOVE-") {
        let v: Vec<&str> = symbol.split('-').collect();
        Some(format!("{}/USD", v[0]))
    } else if symbol.contains("BVOL/") || symbol.contains('/') {
        // BVOL and Spot
        Some(symbol.to_string())
    } else if let Some(pos) = symbol.rfind('-') {
        // linear future
        let base = &symbol[..pos];
        Some(format!("{}/USD", base))
    } else {
        // prediction
        Some(format!("{}/USD", symbol))
    }
}
