use crate::application::repositories::AuctionsRepository;
use foundation::value_objects::Money;
use std::sync::Arc;

pub struct PlacingBidInputDto {}

pub struct PlacingBidOutputDto {
    pub is_winner: bool,
    pub current_price: Money,
}

pub trait PlacingBidOutputBoundary {
    fn present(&mut self, dto: PlacingBidOutputDto);
}

pub struct PlacingBid {
    output_boundary: Arc<dyn PlacingBidOutputBoundary>,
    auctions_repo: Arc<dyn AuctionsRepository>,
}

impl PlacingBid {
    pub fn new(
        output_boundary: Arc<dyn PlacingBidOutputBoundary>,
        auctions_repo: Arc<dyn AuctionsRepository>,
    ) -> Self {
        Self {
            output_boundary,
            auctions_repo,
        }
    }
}
