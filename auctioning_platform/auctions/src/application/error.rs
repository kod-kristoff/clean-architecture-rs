use crate::domain::DomainError;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    RepositoryError,
    UseCaseError,
    Domain(DomainError),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RepositoryError => write!(f, "repository error"),
            Self::UseCaseError => write!(f, "use case error"),
            Self::Domain(err) => write!(f, "Domain error: {}", err),
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain(err) => Some(err),
            _ => None,
        }
    }
}

impl From<DomainError> for ApplicationError {
    fn from(err: DomainError) -> Self {
        Self::Domain(err)
    }
}
