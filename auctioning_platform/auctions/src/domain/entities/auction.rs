use crate::domain::{entities::Bid, value_objects::BidderId};
use foundation::value_objects::Money;

pub struct Auction {
    starting_price: Money,
    bids: Vec<Bid>,
}

impl Auction {
    pub fn new(starting_price: Money) -> Self {
        let bids = Vec::new();
        Self {
            starting_price,
            bids,
        }
    }

    pub fn new_with_bids(starting_price: Money, mut bids: Vec<Bid>) -> Self {
        bids.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
        Self {
            starting_price,
            bids,
        }
    }
}

// Builder lite
impl Auction {
    pub fn with_bids(mut self, mut bids: Vec<Bid>) -> Self {
        bids.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
        self.bids = bids;
        self
    }
}
impl Auction {
    pub fn current_price(&self) -> Money {
        if self.bids.is_empty() {
            self.starting_price
        } else {
            self.highest_bid().amount
        }
    }

    pub fn starting_price(&self) -> Money {
        self.starting_price
    }

    pub fn winners(&self) -> Vec<BidderId> {
        if self.bids.is_empty() {
            Vec::new()
        } else {
            vec![self.highest_bid().bidder_id]
        }
    }

    fn highest_bid(&self) -> &Bid {
        let idx = self.bids.len() - 1;
        &self.bids[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{BidId, BidderId};
    use foundation::value_objects::factories::get_dollars;
    use rstest::{fixture, rstest};

    #[fixture]
    fn auction_wo_bids() -> Auction {
        Auction::new(get_dollars("7.49"))
    }

    mod empty_bid_list {
        use super::*;

        #[rstest]
        fn should_use_starting_price_as_current_price(auction_wo_bids: Auction) {
            assert_eq!(
                auction_wo_bids.current_price(),
                auction_wo_bids.starting_price()
            )
        }

        #[rstest]
        fn should_return_no_winners(auction_wo_bids: Auction) {
            assert_eq!(auction_wo_bids.winners(), Vec::new());
        }
    }
    #[test]
    fn should_return_highest_bid_for_current_price() {
        let auction = Auction::new(get_dollars("10")).with_bids(vec![Bid {
            id: Some(BidId(1)),
            bidder_id: BidderId(1),
            amount: get_dollars("20"),
        }]);

        assert_eq!(auction.current_price(), get_dollars("20"));
    }

    #[test]
    fn untouched_auction_has_current_price_equal_to_starting() {
        let starting_price = get_dollars("12.99");
        let auction = Auction::new(starting_price);

        assert_eq!(auction.current_price(), starting_price);
    }

    #[rstest]
    fn should_return_highest_bids_user_id_for_winners_list(auction_wo_bids: Auction) {
        let auction = auction_wo_bids.with_bids(vec![
            Bid {
                id: Some(BidId(1)),
                bidder_id: BidderId(1),
                amount: get_dollars("101"),
            },
            Bid {
                id: Some(BidId(2)),
                bidder_id: BidderId(2),
                amount: get_dollars("15"),
            },
            Bid {
                id: Some(BidId(3)),
                bidder_id: BidderId(3),
                amount: get_dollars("100"),
            },
        ]);
        assert_eq!(auction.winners(), vec![BidderId(1)]);
    }
}
