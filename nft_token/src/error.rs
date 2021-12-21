use near_contract_standards::non_fungible_token::TokenId;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

pub(crate) type Result<T> = std::result::Result<T, BattlemonError>;

#[derive(Clone)]
pub enum BattlemonError {
    ModelNotFound(TokenId),
    OwnerNotFound(TokenId),
    InstructionError(InstructionErrorKind),
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
            BattlemonError::InstructionError(e) => {
                write!(f, "provided instructions contain errors because: {}", e)
            }
        }
    }
}

#[derive(Clone)]
pub enum InstructionErrorKind {
    NotEqualOwners,
    Empty,
    ChunkBoundsOut,
    IncompatibleModels,
}

impl Display for InstructionErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InstructionErrorKind::NotEqualOwners => {
                write!(f, "owners for token's models must be the same")
            }
            InstructionErrorKind::Empty => {
                write!(f, "empty instructions are not allowed")
            }
            InstructionErrorKind::ChunkBoundsOut => write!(f, "index out of bound in chunk"),
            InstructionErrorKind::IncompatibleModels => write!(f, "models are not compatible"),
        }
    }
}

impl Debug for InstructionErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
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
            BattlemonError::InstructionError(_) => None,
        }
    }
}
