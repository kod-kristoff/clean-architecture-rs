use auctions::application::repositories::AuctionsRepository;
use auctions::application::ApplicationResult;
use auctions::domain::{entities::Auction, value_objects::AuctionId};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct InMemoryAuctionsRepo {
    store: Mutex<HashMap<AuctionId, Auction>>,
}

impl InMemoryAuctionsRepo {
    pub fn new() -> Self {
        let store = Mutex::new(HashMap::new());
        Self { store }
    }
}

impl AuctionsRepository for InMemoryAuctionsRepo {
    fn get(&self, auction_id: AuctionId) -> Option<Auction> {
        match self.store.lock().unwrap().get(&auction_id) {
            None => None,
            Some(auction) => Some(auction.clone()),
        }
    }

    fn save(&self, auction: &Auction) -> ApplicationResult<()> {
        self.store
            .lock()
            .unwrap()
            .insert(auction.id(), auction.clone());
        Ok(())
    }
}
