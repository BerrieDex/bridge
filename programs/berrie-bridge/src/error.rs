use anchor_lang::prelude::*;

#[error_code]
pub enum BerrieBridgeError {
    #[msg("Invalid amount")]
    InvalidAmount,
}
