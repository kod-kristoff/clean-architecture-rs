use crate::domain::value_objects::{BidId, BidderId};
use foundation::value_objects::Money;

#[derive(Clone, Debug, PartialEq)]
pub struct Bid {
    pub id: Option<BidId>,
    pub bidder_id: BidderId,
    pub amount: Money,
}
