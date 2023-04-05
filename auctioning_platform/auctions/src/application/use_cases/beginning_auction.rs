use chrono::{DateTime, Utc};
use foundation::value_objects::Money;

use crate::{
    application::{repositories::DynAuctionsRepository, ApplicationError, ApplicationResult},
    domain::{entities::Auction, value_objects::AuctionId, DomainError},
};

#[derive(Debug)]
pub struct BeginningAuctionInputDto {
    pub auction_id: AuctionId,
    pub title: String,
    pub starting_price: Money,
    pub ends_at: DateTime<Utc>,
}

pub struct BeginningAuction {
    auctions_repo: DynAuctionsRepository,
}

impl BeginningAuction {
    pub fn new(auctions_repo: DynAuctionsRepository) -> Self {
        Self { auctions_repo }
    }
}

impl BeginningAuction {
    pub fn execute(&self, input_dto: BeginningAuctionInputDto) -> ApplicationResult<()> {
        if input_dto.ends_at < Utc::now() {
            return Err(ApplicationError::Domain(
                DomainError::AuctionEndingInThePast,
            ));
        }
        let auction = Auction::new(
            input_dto.auction_id,
            input_dto.title,
            input_dto.starting_price,
            input_dto.ends_at,
        );
        self.auctions_repo.save(&auction)?;
        Ok(())
    }
}
