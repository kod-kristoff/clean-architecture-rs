mod auctions;

use super::ApplicationResult;
use std::sync::Arc;

pub use self::auctions::AuctionsRepository;

pub type DynAuctionsRepository = Arc<dyn AuctionsRepository>;
