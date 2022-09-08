use crate::value_objects::Currency;
use core::str::FromStr;
use rust_decimal::Decimal;
use std::cmp::Ordering;

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

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency != other.currency {
            return None;
        }
        self.amount.partial_cmp(&other.amount)
    }
}

impl std::ops::Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            panic!("currency mismatch");
        }
        Money {
            currency: self.currency,
            amount: self.amount + rhs.amount,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::USD;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case(Money::new(USD, "18.59"), "18.59 $")]
    fn display(#[case] money_instance: Money, #[case] expected_str: &str) {
        assert_eq!(format!("{}", money_instance), expected_str);
    }
}
