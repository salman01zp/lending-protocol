// Utility functions

/// Convert asset ID to name
pub fn asset_id_to_name(asset_id: u32) -> &'static str {
    match asset_id {
        1 => "USDC",
        2 => "DAI",
        3 => "WETH",
        4 => "WBTC",
        _ => "UNKNOWN",
    }
}

/// Format price with 8 decimals
pub fn format_price(price: u64) -> String {
    let dollars = price / 100_000_000;
    let cents = price % 100_000_000;
    format!("${}.{:08}", dollars, cents)
}

/// Calculate health factor display value
pub fn format_health_factor(health_factor: u64) -> String {
    let precision = 10000;
    if health_factor >= 1_000_000 {
        "∞ (no debt)".to_string()
    } else {
        let integer = health_factor / precision;
        let decimal = health_factor % precision;
        format!("{}.{:04}", integer, decimal)
    }
}

/// Format basis points to percentage
pub fn basis_points_to_percentage(basis_points: u64) -> String {
    let percentage = (basis_points as f64) / 100.0;
    format!("{:.2}%", percentage)
}
