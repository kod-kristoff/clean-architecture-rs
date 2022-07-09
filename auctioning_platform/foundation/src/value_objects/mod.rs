mod money;
mod currency;

use std::sync::Arc;

pub use money::Money;
pub type Currency = Arc<dyn currency::Currency>;
pub const USD: Arc<dyn currency::Currency> = Arc::new(currency::USD);
