use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;
use num_derive::{FromPrimitive};

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PoolError {

    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Invalid module")]
    InvalidModule,

}

impl From<PoolError> for ProgramError {
    fn from(e: PoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PoolError {
    fn type_of() -> &'static str {
        "PoolError"
    }
}