use auctions::application::use_cases::placing_bid::{
    PlaceBid, PlacingBidOutputBoundary, PlacingBidOutputDto,
};
use auctions::application::repositories::AuctionsRepository;
use auctions::domain::entities::Auction;
use auctions::domain::value_objects::{AuctionId, BidderId};
use auctions::PlacingBid;
use foundation::value_objects::factories::get_dollars;
use rstest::{fixture, rstest};
use std::sync::Arc;

struct PlacingBidOutputBoundaryFake {
    dto: Option<PlacingBidOutputDto>,
}

impl PlacingBidOutputBoundaryFake {
    fn new() -> Self {
        Self { dto: None }
    }
}

impl PlacingBidOutputBoundary for PlacingBidOutputBoundaryFake {
    fn present(&mut self, dto: PlacingBidOutputDto) {
        self.dto = Some(dto);
    }
}
#[fixture]
fn output_boundary() -> PlacingBidOutputBoundaryFake {
    PlacingBidOutputBoundaryFake::new()
}

#[fixture]
fn auctions_repo() -> Arc<dyn AuctionsRepository> {
    todo!("")
}

#[fixture]
fn auction() -> Auction {
    Auction::new(AuctionId(1), get_dollars("7.49"))
}

#[fixture]
fn auction_id(auction: Auction) -> AuctionId {
    auction.id()
}
#[fixture]
fn place_bid_uc(
    auction: Auction,
    auctions_repo: Arc<dyn AuctionsRepository>,
) -> PlacingBid {
    auctions_repo.save(&auction);
    PlacingBid::new(auctions_repo)
}

#[rstest]
fn auction_FirstBidHigherThanIntialPrice_IsWinning(
    place_bid_uc: PlacingBid,
    auction_id: AuctionId,
    auctions_repo: Arc<dyn AuctionsRepository>,
) {
    place_bid_uc.execute(PlaceBid::new(BidderId(1), auction_id, get_dollars("100")));

    assert!(auctions_repo.get(auction_id).is_some());
}
