use crate::domain::{
    entities::Bid,
    error::DomainError,
    events::DomainEvent,
    value_objects::{AuctionId, BidderId},
};
use chrono::{DateTime, Utc};
use foundation::value_objects::Money;

#[derive(Clone, Debug)]
pub struct Auction {
    id: AuctionId,
    title: String,
    starting_price: Money,
    ends_at: DateTime<Utc>,
    bids: Vec<Bid>,
}

impl Auction {
    pub fn new(
        id: AuctionId,
        title: String,
        starting_price: Money,
        ends_at: DateTime<Utc>,
    ) -> Self {
        let bids = Vec::new();
        Self {
            id,
            title,
            starting_price,
            ends_at,
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
    pub fn id(&self) -> AuctionId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn current_price(&self) -> Money {
        if self.bids.is_empty() {
            self.starting_price
        } else {
            self.highest_bid().amount
        }
    }

    pub fn place_bid(
        &mut self,
        bidder_id: BidderId,
        amount: Money,
    ) -> Result<Vec<DomainEvent>, DomainError> {
        if Utc::now() > self.ends_at {
            return Err(DomainError::BidOnEndedAuction);
        }
        let mut events = vec![];
        if amount > self.current_price() {
            let new_bid = Bid {
                id: None,
                bidder_id,
                amount,
            };
            self.bids.push(new_bid);
            events.push(DomainEvent::WinningBidPlaced {
                auction_id: self.id,
                bidder_id,
                bid_amount: amount,
                auction_title: self.title.clone(),
            })
        }
        Ok(events)
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
    use crate::domain::{
        events::DomainEvent::WinningBidPlaced,
        value_objects::{BidId, BidderId},
    };
    use chrono::Duration;
    use foundation::value_objects::factories::get_dollars;
    use rstest::{fixture, rstest};

    #[fixture]
    fn ends_at() -> DateTime<Utc> {
        Utc::now() + Duration::days(7)
    }

    #[fixture]
    fn auction_wo_bids(#[default("socks")] title: &str, ends_at: DateTime<Utc>) -> Auction {
        Auction::new(
            AuctionId(1),
            title.to_string(),
            get_dollars("7.49"),
            ends_at,
        )
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

    mod place_bid {
        use super::*;

        #[rstest]
        fn should_win_auction_if_is_the_only_bidder_above_starting_price(
            mut auction_wo_bids: Auction,
        ) {
            auction_wo_bids.place_bid(BidderId(1), get_dollars("11"));

            assert_eq!(auction_wo_bids.winners(), vec![BidderId(1)]);
        }

        #[rstest]
        fn should_not_be_winning_auction_if_bids_below_starting_price(
            mut auction_wo_bids: Auction,
        ) {
            auction_wo_bids.place_bid(BidderId(1), get_dollars("5"));
            assert_eq!(auction_wo_bids.winners(), vec![]);
        }
    }
    #[rstest]
    fn should_return_highest_bid_for_current_price(auction_wo_bids: Auction) {
        let auction = auction_wo_bids.with_bids(vec![Bid {
            id: Some(BidId(1)),
            bidder_id: BidderId(1),
            amount: get_dollars("20"),
        }]);

        assert_eq!(auction.current_price(), get_dollars("20"));
    }

    #[rstest]
    fn untouched_auction_has_current_price_equal_to_starting(ends_at: DateTime<Utc>) {
        let starting_price = get_dollars("12.99");
        let auction = Auction::new(AuctionId(1), "shoes".to_string(), starting_price, ends_at);

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
    mod events {
        use super::*;

        #[rstest]
        fn should_emit_winning_event_if_the_first_offer(mut auction_wo_bids: Auction) {
            let winning_amount = auction_wo_bids.current_price() + get_dollars("1.00");
            let domain_events = auction_wo_bids
                .place_bid(BidderId(1), winning_amount)
                .unwrap();
            assert_eq!(
                domain_events,
                vec![WinningBidPlaced {
                    auction_id: auction_wo_bids.id(),
                    bidder_id: BidderId(1),
                    bid_amount: winning_amount,
                    auction_title: auction_wo_bids.title().to_string(),
                }]
            );
        }
    }
}
