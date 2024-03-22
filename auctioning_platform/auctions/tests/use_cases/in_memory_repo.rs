use auctions::application::repositories::AuctionsRepository;
use auctions::application::ApplicationResult;
use auctions::domain::{entities::Auction, value_objects::AuctionId};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct InMemoryAuctionsRepo {
    store: Mutex<HashMap<AuctionId, Auction>>,
}
impl Default for InMemoryAuctionsRepo {
    fn default() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}
impl InMemoryAuctionsRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AuctionsRepository for InMemoryAuctionsRepo {
    fn get(&self, auction_id: AuctionId) -> Option<Auction> {
        self.store.lock().unwrap().get(&auction_id).cloned()
    }

    fn save(&self, auction: &Auction) -> ApplicationResult<()> {
        self.store
            .lock()
            .unwrap()
            .insert(auction.id(), auction.clone());
        Ok(())
    }
}
