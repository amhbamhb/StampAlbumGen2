pub mod page;

use super::Result;

fn arguments<const N: usize>(name: &str, values: &[f64], positive: bool) -> Result<[f64; N]> {
    let values: [f64; N] = values
        .try_into()
        .map_err(|_| format!("{name} requires exactly {N} numeric arguments"))?;
    for (index, value) in values.iter().enumerate() {
        if !value.is_finite() || *value < 0.0 || (positive && *value == 0.0) {
            let range = if positive { "positive" } else { "nonnegative" };
            return Err(format!(
                "{name} argument {} must be finite and {range}",
                index + 1
            ));
        }
    }
    Ok(values)
}
