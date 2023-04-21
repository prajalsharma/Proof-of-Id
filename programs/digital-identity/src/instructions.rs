use crate::state::{DigitalIdentity, DigitalProofs};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateIdentity<'info> {
    #[account(init,payer=authority,seeds=["dig_identity".as_bytes(),authority.key().as_ref()],bump,space=DigitalIdentity::space())]
    pub dig_identity_acc: Account<'info, DigitalIdentity>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateIdentityProofs<'info> {
    #[account(init,payer=authority,seeds=["dig_proof".as_bytes(),dig_identity_acc.key().as_ref()],bump,space=DigitalProofs::space())]
    pub dig_proofs_acc: Account<'info, DigitalProofs>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one=authority)]
    pub dig_identity_acc: Account<'info, DigitalIdentity>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyIdentity<'info> {
    pub dig_identity_acc: Account<'info, DigitalIdentity>,
    /// CHECK:safe
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct VerifyProofs<'info> {
    pub dig_identity_acc: Account<'info, DigitalIdentity>,
    pub dig_proofs_acc: Account<'info, DigitalProofs>,
    /// CHECK:safe
    pub authority: AccountInfo<'info>,
}
