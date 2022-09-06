pub mod application;
pub mod domain;

pub use crate::application::use_cases::placing_bid::PlacingBid;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
