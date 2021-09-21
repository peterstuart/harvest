use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
#[serde(transparent)]
pub struct Hours(Decimal);

impl std::fmt::Debug for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.0.floor();
        let minutes = (self.0 - hours) * dec!(60);

        format!("{}:{:0>2}", hours, minutes.round()).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Hours(dec!(1)).to_string(), "1:00");
        assert_eq!(Hours(dec!(1.5)).to_string(), "1:30");
        assert_eq!(Hours(dec!(1.51)).to_string(), "1:31");
        assert_eq!(Hours(dec!(10)).to_string(), "10:00");
    }
}
