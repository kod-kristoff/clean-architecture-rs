use auctions::application::use_cases::placing_bid::{
    PlacingBidInputDto, PlacingBidOutputBoundary, PlacingBidOutputDto,
};
use auctions::domain::value_objects::AuctionId;
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
fn output_boundary() -> Arc<dyn PlacingBidOutputBoundary> {
    Arc::new(PlacingBidOutputBoundaryFake::new())
}

#[fixture]
fn place_bid_uc(
    output_boundary: Arc<dyn PlacingBidOutputBoundary>,
    auction: Auction,
    mut auctions_repo: AuctionsRepository,
) -> PlacingBid {
    auctions_repo.save(&auction);
    PlacingBid::new(output_boundary, auctions_repo)
}

#[rstest]
fn auction_FirstBidHigherThanIntialPrice_IsWinning(
    place_bid_uc: PlacingBid,
    output_boundary: Arc<dyn PlacingBidOutputBoundary>,
    auction_id: AuctionId,
) {
    place_bid_uc.execute(PlacingBidInputDto::new(1, auction_id, get_dollars("100")));

    let expected_dto = PlacingBidOutputDto {
        is_winner: true,
        current_price: get_dollars("100"),
    };
    assert_eq!(output_boundary.dto, expected_dto);
}
