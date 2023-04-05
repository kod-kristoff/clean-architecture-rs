mod error;
pub mod repositories;
pub mod use_cases;

pub use error::ApplicationError;

pub type ApplicationResult<T> = Result<T, ApplicationError>;
