use super::value_objects::{AuctionId, BidderId};
use foundation::value_objects::Money;

#[derive(Debug, PartialEq)]
pub enum DomainEvent {
    WinningBidPlaced {
        auction_id: AuctionId,
        bidder_id: BidderId,
        bid_amount: Money,
        auction_title: String,
    },
}
