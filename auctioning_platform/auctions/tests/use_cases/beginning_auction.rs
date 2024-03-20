use std::error::Error;

use crate::factories::auctions_repo;
use auctions::{
    application::repositories::DynAuctionsRepository, domain::DomainError, BeginningAuction,
    BeginningAuctionInputDto,
};
use chrono::Utc;
use foundation::value_objects::factories::get_dollars;
use rstest::{fixture, rstest};

use crate::factories;

#[fixture]
fn beginning_auction_uc(auctions_repo: DynAuctionsRepository) -> BeginningAuction {
    BeginningAuction::new(auctions_repo)
}

#[rstest]
fn ends_in_the_past_returns_error(beginning_auction_uc: BeginningAuction) {
    let input = BeginningAuctionInputDto {
        auction_id: 1.into(),
        title: "Socks".into(),
        starting_price: get_dollars("1.00"),
        ends_at: factories::yesterday(),
    };
    let res = beginning_auction_uc.execute(input);
    assert_eq!(
        res.unwrap_err().source().unwrap().to_string(),
        DomainError::AuctionEndingInThePast.to_string()
    );
}

#[rstest]
fn ends_in_the_future_returns_ok(auctions_repo: DynAuctionsRepository) {
    let uc = BeginningAuction::new(auctions_repo.clone());
    let input = BeginningAuctionInputDto {
        auction_id: 1.into(),
        title: "Socks".into(),
        starting_price: get_dollars("1.00"),
        ends_at: Utc::now() + chrono::Duration::try_days(7).unwrap(),
    };
    let auction_id = input.auction_id;
    uc.execute(input).unwrap();
    // assert_eq!(
    //     res.unwrap_err().source().unwrap().to_string(),
    //     DomainError::AuctionEndingInThePast.to_string()
    // );
    assert!(auctions_repo.get(auction_id).is_some());
}
