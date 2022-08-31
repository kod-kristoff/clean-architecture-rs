use foundation::value_objects::Money;

pub struct Auction {
    starting_price: Money,
}

impl Auction {
    pub fn new(starting_price: Money) -> Self {
        Self { starting_price }
    }
}
impl Auction {
    fn current_price(&self) -> Money {
        self.starting_price
    }

    fn starting_price(&self) -> Money {
        self.starting_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use foundation::value_objects::USD;
    use rstest::{fixture, rstest};

    #[fixture]
    fn auction_wo_bids() -> Auction {
        Auction {
            starting_price: Money::new(USD, "7.49"),
        }
    }
    #[rstest]
    fn should_use_starting_price_as_current_price_for_empty_bids_list(auction_wo_bids: Auction) {
        assert_eq!(
            auction_wo_bids.current_price(),
            auction_wo_bids.starting_price()
        )
    }

    #[test]
    fn untouched_auction_has_current_price_equal_to_starting() {
        let starting_price = Money::new(USD, "12.99");
        let auction = Auction::new(starting_price);

        assert_eq!(auction.current_price(), starting_price);
    }
}
