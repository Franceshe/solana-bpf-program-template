// program specific errors

// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;



/// we are defining an error type here.
///ref: https://docs.rs/thiserror/latest/thiserror/ and
/// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
/// 

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
/// reference: https://doc.rust-lang.org/std/convert/trait.From.html
/// We are implementing a generic trait, specifically the From (opens new window)trait which the ? operator wants to use.
/// The reason we do this conversion in the first place is that the entrypoint returns a Result of either nothing or a ProgramError
}