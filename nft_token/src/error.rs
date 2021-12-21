use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

pub(crate) type Result<T> = std::result::Result<T, BattlemonError>;

pub enum BattlemonError {
    ModelNotFound(String),
    OwnerNotFound(String),
    PutSlotError(String),
}

impl Display for BattlemonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BattlemonError::ModelNotFound(token_id) => {
                write!(f, "couldn't find the model for token with id: {}", token_id)
            }
            BattlemonError::OwnerNotFound(owner_id) => {
                write!(f, "couldn't find owner with id: {}", owner_id)
            }

            BattlemonError::PutSlotError(e) => {
                write!(f, "couldn't put slot into body because: {}", e)
            }
        }
    }
}

impl Debug for BattlemonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Error for BattlemonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BattlemonError::ModelNotFound(_) => None,
            BattlemonError::OwnerNotFound(_) => None,
            BattlemonError::PutSlotError(_) => None,
        }
    }
}
