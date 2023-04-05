use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DomainError {
    BidOnEndedAuction,
    AuctionEndingInThePast,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BidOnEndedAuction => write!(f, "Auction has already ended"),
            Self::AuctionEndingInThePast => write!(f, "Auction can't end in the past."),
        }
    }
}

impl std::error::Error for DomainError {}
