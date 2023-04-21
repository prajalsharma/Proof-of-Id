use anchor_lang::prelude::*;

#[error_code]
pub enum DigitalIdentityErrorCode {
    #[msg("Digital Identity Account cannot be verified")]
    DigitalIdentityDoesntMatchWithAuth,
    #[msg("Digital Proofs Account cannot be verified")]
    DigitalProofDoesntMatchWithAuth,
}
