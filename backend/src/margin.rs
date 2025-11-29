use anyhow::{Result, anyhow};

/// Fixed-point SCALE used across the project (1 unit = 1/SCALE)
pub const SCALE: u128 = 1_000_000u128;
pub const SCALE_F64: f64 = SCALE as f64;

/// Calculate initial margin (scaled)
/// position_size_usdt_scaled : position size in USDT scaled by SCALE (e.g. 1000 USDT -> 1000 * SCALE)
/// leverage : e.g. 10 for 10x
pub fn calculate_initial_margin(position_size_usdt_scaled: u128, leverage: u128) -> Result<u128> {
    if leverage == 0 {
        return Err(anyhow!("leverage must be > 0"));
    }
    // initial_margin = (position_size * SCALE) / leverage
    let v = position_size_usdt_scaled
        .checked_mul(SCALE).ok_or_else(|| anyhow!("overflow mul"))?
        .checked_div(leverage).ok_or_else(|| anyhow!("div by zero"))?;
    Ok(v)
}

/// Convert human float USDT -> scaled integer
pub fn to_scaled(amount: f64) -> u128 {
    ((amount * SCALE_F64).round()) as u128
}

/// Convert scaled integer -> human float USDT
pub fn from_scaled(scaled: u128) -> f64 {
    (scaled as f64) / SCALE_F64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_margin_simple() {
        // 1000 USDT position, 10x -> initial margin = 100 USDT
        let pos = to_scaled(1000.0);
        let margin_scaled = calculate_initial_margin(pos, 10).unwrap();
        let margin = from_scaled(margin_scaled);
        assert!((margin - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_to_from_scaled() {
        let v = 1234.567;
        let s = to_scaled(v);
        let back = from_scaled(s);
        assert!((v - back).abs() < 1e-6);
    }
}
