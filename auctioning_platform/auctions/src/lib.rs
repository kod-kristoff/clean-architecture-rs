#![allow(dead_code)]
#![allow(unused)]
pub mod application;
pub mod domain;

pub use crate::application::use_cases::placing_bid::PlacingBid;

pub use crate::application::use_cases::{BeginningAuction, BeginningAuctionInputDto};
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
