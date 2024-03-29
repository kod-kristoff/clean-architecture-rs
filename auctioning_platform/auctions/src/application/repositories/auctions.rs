use super::ApplicationResult;
use crate::domain::entities::Auction;
use crate::domain::value_objects::AuctionId;

pub trait AuctionsRepository {
    fn get(&self, auction_id: AuctionId) -> Option<Auction>;
    fn save(&self, auction: &Auction) -> ApplicationResult<()>;
}
