use crate::domain::{
    entities::Bid,
    error::DomainError,
    events::{Event, WinningBidPlaced},
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

impl Auction {
    // use cases
    pub fn place_bid(
        &mut self,
        bidder_id: BidderId,
        amount: Money,
    ) -> Result<Vec<Box<dyn Event>>, DomainError> {
        if Utc::now() > self.ends_at {
            return Err(DomainError::BidOnEndedAuction);
        }
        let mut events: Vec<Box<dyn Event>> = Vec::new();
        if amount > self.current_price() {
            let new_bid = Bid {
                id: None,
                bidder_id,
                amount,
            };
            self.bids.push(new_bid);
            events.push(Box::new(WinningBidPlaced {
                auction_id: self.id,
                bidder_id,
                bid_amount: amount,
                auction_title: self.title.clone(),
            }))
        }
        Ok(events)
    }
}

#[cfg(test)]
mod tests;
