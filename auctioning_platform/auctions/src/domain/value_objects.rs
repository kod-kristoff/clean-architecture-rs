#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AuctionId(pub u32);

impl From<u32> for AuctionId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BidId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BidderId(pub u32);
