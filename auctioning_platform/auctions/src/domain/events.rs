use std::any::TypeId;

use super::value_objects::{AuctionId, BidderId};
pub use foundation::events::Event;
use foundation::value_objects::Money;

#[derive(Debug, PartialEq)]
pub struct WinningBidPlaced {
    pub auction_id: AuctionId,
    pub bidder_id: BidderId,
    pub bid_amount: Money,
    pub auction_title: String,
}

impl Event for WinningBidPlaced {
    fn event_id(&self) -> std::any::TypeId {
        TypeId::of::<Self>()
    }
}
