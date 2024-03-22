use crate::factories::{self, auctions_repo};
use auctions::application::repositories::{AuctionsRepository, DynAuctionsRepository};
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
    Utc::now() + chrono::Duration::try_days(7).unwrap()
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
    auctions_repo.save(&auction).expect("saving auction");
    PlacingBid::new(auctions_repo)
}

#[rstest]
fn first_bid_higher_than_initial_price_is_winning(
    auction: Auction,
    auction_id: AuctionId,
    auctions_repo: DynAuctionsRepository,
) {
    let uc = place_bid_uc(auction, auctions_repo.clone());
    let output = uc
        .execute(PlaceBid::new(BidderId(1), auction_id, get_dollars("100")))
        .unwrap();

    assert!(output.is_winner);

    let auction = auctions_repo.get(auction_id);
    println!("{:?}", auction);
    assert!(auctions_repo.get(auction_id).is_some());
}

#[rstest]
fn bid_lower_than_current_price_is_losing(place_bid_uc: PlacingBid, auction_id: AuctionId) {
    let dto = place_bid_uc
        .execute(PlaceBid::new(1.into(), auction_id, get_dollars("5")))
        .unwrap();

    assert!(!dto.is_winner);
    assert_eq!(dto.current_price, get_dollars("7.49"));
}

#[rstest]
fn overbid_is_winning(place_bid_uc: PlacingBid, auction_id: AuctionId) {
    place_bid_uc
        .execute(PlaceBid::new(1.into(), auction_id, get_dollars("100")))
        .unwrap();

    let dto = place_bid_uc
        .execute(PlaceBid::new(2.into(), auction_id, get_dollars("120")))
        .unwrap();

    assert_eq!(
        dto,
        PlacingBidOutputDto {
            is_winner: true,
            current_price: get_dollars("120")
        }
    );
}

#[rstest]
fn overbid_by_winner_is_winning(place_bid_uc: PlacingBid, auction_id: AuctionId) {
    place_bid_uc
        .execute(PlaceBid::new(1.into(), auction_id, get_dollars("100")))
        .unwrap();

    let dto = place_bid_uc
        .execute(PlaceBid::new(1.into(), auction_id, get_dollars("120")))
        .unwrap();

    assert_eq!(
        dto,
        PlacingBidOutputDto {
            is_winner: true,
            current_price: get_dollars("120")
        }
    );
}
#[rstest]
fn bid_on_ended_auction_returns_error(
    #[with(factories::yesterday())] auction: Auction,
    auction_id: AuctionId,
    auctions_repo: DynAuctionsRepository,
) {
    println!("{:?}", auction);
    let uc = place_bid_uc(auction, auctions_repo.clone());
    let res = uc.execute(PlaceBid::new(BidderId(1), auction_id, get_dollars("100")));
    assert_eq!(
        res.unwrap_err().source().unwrap().to_string(),
        DomainError::BidOnEndedAuction.to_string()
    );
}
