use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotOwner,
    NotOracle,
    WaveAlreadyFinalized,
    WaveNotFinalized,
    WaveAlreadyExists,
    WaveNotFound,
    ZeroBudget,
    ZeroPoints,
    ZeroAddress,
    AlreadyClaimed,
    ContractPaused,
    ZeroReward,
    InsufficientBalance,
    TokenTransferFailed,
    InvalidTimestamp,
    BudgetExceeded,
    Unauthorized,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotOwner => write!(f, "NotOwner"),
            Self::NotOracle => write!(f, "NotOracle"),
            Self::WaveAlreadyFinalized => write!(f, "WaveAlreadyFinalized"),
            Self::WaveNotFinalized => write!(f, "WaveNotFinalized"),
            Self::WaveAlreadyExists => write!(f, "WaveAlreadyExists"),
            Self::WaveNotFound => write!(f, "WaveNotFound"),
            Self::ZeroBudget => write!(f, "ZeroBudget"),
            Self::ZeroPoints => write!(f, "ZeroPoints"),
            Self::ZeroAddress => write!(f, "ZeroAddress"),
            Self::AlreadyClaimed => write!(f, "AlreadyClaimed"),
            Self::ContractPaused => write!(f, "ContractPaused"),
            Self::ZeroReward => write!(f, "ZeroReward"),
            Self::InsufficientBalance => write!(f, "InsufficientBalance"),
            Self::TokenTransferFailed => write!(f, "TokenTransferFailed"),
            Self::InvalidTimestamp => write!(f, "InvalidTimestamp"),
            Self::BudgetExceeded => write!(f, "BudgetExceeded"),
            Self::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl From<Error> for Vec<u8> {
    fn from(err: Error) -> Vec<u8> {
        err.to_string().into_bytes()
    }
}
