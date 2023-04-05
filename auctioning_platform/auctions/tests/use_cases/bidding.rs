use crate::factories::{self, auctions_repo};
use auctions::application::repositories::AuctionsRepository;
use auctions::application::use_cases::placing_bid::{
    PlaceBid, PlacingBidOutputBoundary, PlacingBidOutputDto,
};
use auctions::domain::entities::Auction;
use auctions::domain::value_objects::{AuctionId, BidderId};
use auctions::domain::DomainError;
use auctions::PlacingBid;
use chrono::{DateTime, Utc};
use foundation::value_objects::factories::get_dollars;
use rstest::{fixture, rstest};
use std::error::Error;
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
fn default_auction_end() -> DateTime<Utc> {
    Utc::now() + chrono::Duration::days(7)
}

#[fixture]
fn auction(default_auction_end: DateTime<Utc>) -> Auction {
    println!("ends_at: {:?}", default_auction_end);
    Auction::new(
        AuctionId(1),
        "socks".into(),
        get_dollars("7.49"),
        default_auction_end,
    )
}

#[fixture]
fn auction_id(auction: Auction) -> AuctionId {
    auction.id()
}
#[fixture]
fn place_bid_uc(auction: Auction, auctions_repo: Arc<dyn AuctionsRepository>) -> PlacingBid {
    auctions_repo.save(&auction);
    PlacingBid::new(auctions_repo)
}

#[rstest]
fn auction_firstbidhigherthanintialprice_is_winning(
    auction: Auction,
    auction_id: AuctionId,
    auctions_repo: Arc<dyn AuctionsRepository>,
) {
    let uc = place_bid_uc(auction, auctions_repo.clone());
    uc.execute(PlaceBid::new(BidderId(1), auction_id, get_dollars("100")))
        .unwrap();

    let auction = auctions_repo.get(auction_id);
    println!("{:?}", auction);
    assert!(auctions_repo.get(auction_id).is_some());
}

#[rstest]
fn bid_on_ended_auction_returns_error(
    #[with(factories::yesterday())] auction: Auction,
    auction_id: AuctionId,
    auctions_repo: Arc<dyn AuctionsRepository>,
) {
    println!("{:?}", auction);
    let uc = place_bid_uc(auction, auctions_repo.clone());
    let res = uc.execute(PlaceBid::new(BidderId(1), auction_id, get_dollars("100")));
    assert_eq!(
        res.unwrap_err().source().unwrap().to_string(),
        DomainError::BidOnEndedAuction.to_string()
    );
}
