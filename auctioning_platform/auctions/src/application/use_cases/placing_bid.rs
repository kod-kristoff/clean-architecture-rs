use crate::application::repositories::AuctionsRepository;
use crate::application::ApplicationResult;
use crate::domain::{
    value_objects::{AuctionId, BidderId},
    DomainError,
};
use foundation::value_objects::Money;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct PlaceBid {
    bidder_id: BidderId,
    auction_id: AuctionId,
    amount: Money,
}

impl PlaceBid {
    pub fn new(bidder_id: BidderId, auction_id: AuctionId, amount: Money) -> Self {
        Self {
            bidder_id,
            auction_id,
            amount,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacingBidOutputDto {
    pub is_winner: bool,
    pub current_price: Money,
}

pub trait PlacingBidOutputBoundary {
    fn present(&mut self, dto: PlacingBidOutputDto);
}

pub struct PlacingBid {
    // output_boundary: Arc<dyn PlacingBidOutputBoundary>,
    auctions_repo: Arc<dyn AuctionsRepository>,
}

impl PlacingBid {
    pub fn new(
        // output_boundary: Arc<dyn PlacingBidOutputBoundary>,
        auctions_repo: Arc<dyn AuctionsRepository>,
    ) -> Self {
        Self {
            // output_boundary,
            auctions_repo,
        }
    }
}

impl PlacingBid {
    pub fn execute(&self, cmd: PlaceBid) -> ApplicationResult<PlacingBidOutputDto> {
        let mut auction = self.auctions_repo.get(cmd.auction_id).unwrap();
        auction.place_bid(cmd.bidder_id, cmd.amount)?;
        self.auctions_repo.save(&auction)?;

        Ok(PlacingBidOutputDto {
            is_winner: auction.winners().contains(&cmd.bidder_id),
            current_price: auction.current_price(),
        })
    }
}
