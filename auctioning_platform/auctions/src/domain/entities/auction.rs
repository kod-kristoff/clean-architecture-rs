use foundation::value_objects::Money;

pub struct Auction {}

impl Auction {
    fn current_price(&self) -> Money {
        todo!()
    }

    fn starting_price(&self) -> Money {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn auction_wo_bids() -> Auction {
        Auction {}
    }
    #[rstest]
    fn should_use_starting_price_as_current_price_for_empty_bids_list(auction_wo_bids: Auction) {
        assert_eq!(auction_wo_bids.current_price(), auction_wo_bids.starting_price())
    }
}
