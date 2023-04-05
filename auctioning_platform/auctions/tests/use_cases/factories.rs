use std::sync::Arc;

use auctions::application::repositories::DynAuctionsRepository;
use chrono::{DateTime, Utc};
use rstest::fixture;

use crate::in_memory_repo::InMemoryAuctionsRepo;

#[fixture]
pub fn yesterday() -> DateTime<Utc> {
    Utc::now() - chrono::Duration::days(1)
}

#[fixture]
pub fn auctions_repo() -> DynAuctionsRepository {
    Arc::new(InMemoryAuctionsRepo::new())
}
