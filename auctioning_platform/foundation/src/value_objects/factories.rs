use crate::value_objects::{Money, USD};

pub fn get_dollars(amount: &str) -> Money {
    Money::new(USD, amount)
}
