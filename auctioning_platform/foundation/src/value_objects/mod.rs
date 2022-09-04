mod currency;
pub mod factories;
mod money;

use std::sync::Arc;

pub use currency::{Currency, USD};
pub use money::Money;
// pub type Currency = Arc<dyn currency::Currency>;
// pub const USD: Arc<dyn currency::Currency> = Arc::new(currency::USD);
