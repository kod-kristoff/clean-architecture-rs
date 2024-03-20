use super::*;
use crate::domain::value_objects::{BidId, BidderId};
use chrono::Duration;
use foundation::value_objects::factories::get_dollars;
use rstest::{fixture, rstest};

#[fixture]
fn ends_at() -> DateTime<Utc> {
    Utc::now() + Duration::try_days(7).unwrap()
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
        auction_wo_bids
            .place_bid(BidderId(1), get_dollars("11"))
            .unwrap();

        assert_eq!(auction_wo_bids.winners(), vec![BidderId(1)]);
    }

    #[rstest]
    fn should_not_be_winning_auction_if_bids_below_starting_price(
        mut auction_wo_bids: Auction,
    ) {
        auction_wo_bids
            .place_bid(BidderId(1), get_dollars("5"))
            .unwrap();
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
        assert_eq!(domain_events.len(), 1);
        let expected_event = WinningBidPlaced {
            auction_id: auction_wo_bids.id(),
            bidder_id: BidderId(1),
            bid_amount: winning_amount,
            auction_title: auction_wo_bids.title().to_string(),
        };
        assert_eq!(domain_events[0].event_id(), expected_event.event_id());
    }
}
