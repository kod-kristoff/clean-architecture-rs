use rust_decimal::Decimal;
use core::str::FromStr;
use crate::value_objects::{Currency, USD};

#[derive(Debug, PartialEq)]
pub struct Money {
    currency: Currency,
    amount: Decimal,
}

impl Money {
    pub fn new(currency: Currency, amount_str: &str) -> Self {
        let amount = Decimal::from_str(amount_str).unwrap()
        Self { currency, amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case(Money::new(USD, "18.59"), "Money { currency: USD, amount: Decimal('18.59') }")]
    fn debug(#[case] money_instance: Money, #[case] expected_str: &str) {
        assert_eq!(format!("{:?}", money_instance), expected_str);
    }
}
