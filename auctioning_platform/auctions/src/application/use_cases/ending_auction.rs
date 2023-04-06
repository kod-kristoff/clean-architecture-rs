use crate::{
    application::{repositories::DynAuctionsRepository, ApplicationResult},
    domain::value_objects::AuctionId,
};

#[derive(Debug)]
pub struct EndAuction {
    pub auction_id: AuctionId,
}

pub struct EndingAuction {
    auctions_repo: DynAuctionsRepository,
}

impl EndingAuction {
    pub fn new(auctions_repo: DynAuctionsRepository) -> Self {
        Self { auctions_repo }
    }

    pub fn execute(&self, cmd: EndAuction) -> ApplicationResult<()> {
        todo!()
    }
}
