use crate::value_objects::{Currency, USD};
use core::str::FromStr;
use rust_decimal::Decimal;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Money {
    currency: Currency,
    amount: Decimal,
}

impl Money {
    pub fn new(currency: Currency, amount_str: &str) -> Self {
        let amount = Decimal::from_str(amount_str).unwrap();
        Self { currency, amount }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case(Money::new(USD, "18.59"), "18.59 $")]
    fn display(#[case] money_instance: Money, #[case] expected_str: &str) {
        assert_eq!(format!("{}", money_instance), expected_str);
    }
}
