use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;
use errors::*;
use instructions::*;
use state::*;

declare_id!("7E6f2pscJXdX8Jb1jbxE4ir5LPEqTH3FiLg4HQXTU11P");

#[program]
pub mod digital_identity {

    use super::*;

    pub fn create_identity(
        ctx: Context<CreateIdentity>,
        create_identity_params: DigitalIdentityParam,
    ) -> Result<()> {
        let ref mut dig_identity_acc = ctx.accounts.dig_identity_acc;
        dig_identity_acc.authority = ctx.accounts.authority.key();
        dig_identity_acc.name = create_identity_params.name;
        dig_identity_acc.contact_number = create_identity_params.contact_number;
        dig_identity_acc.dob = create_identity_params.dob;
        dig_identity_acc.residence_address = create_identity_params.residence_address;
        dig_identity_acc.pan_number = create_identity_params.pan_number;
        dig_identity_acc.aadhar_number = create_identity_params.aadhar_number;
        dig_identity_acc.passport_id = create_identity_params.passport_id;
        dig_identity_acc.aadhar_attached = false;
        dig_identity_acc.pan_attached = false;
        dig_identity_acc.pic_attached = false;
        dig_identity_acc.pan_attached = false;
        Ok(())
    }

    pub fn create_proofs(
        ctx: Context<CreateIdentityProofs>,
        create_proofs_param: DigiatProofsParam,
    ) -> Result<()> {
        let ref mut dig_proofs_acc = ctx.accounts.dig_proofs_acc;
        let ref mut dig_identity_acc = ctx.accounts.dig_identity_acc;
        dig_proofs_acc.authority = ctx.accounts.authority.key();
        dig_proofs_acc.aadhar_upload = create_proofs_param.aadhar_upload;
        dig_proofs_acc.pan_upload = create_proofs_param.pan_upload;
        dig_proofs_acc.passport_upload = create_proofs_param.passport_upload;
        dig_proofs_acc.picture_upload = create_proofs_param.picture_upload;
        dig_identity_acc.aadhar_attached = true;
        dig_identity_acc.pan_attached = true;
        dig_identity_acc.passport_attached = true;
        dig_identity_acc.pic_attached = true;
        Ok(())
    }

    pub fn verify_identity(ctx: Context<VerifyIdentity>) -> Result<()> {
        let ref dig_identity_acc = ctx.accounts.dig_identity_acc;
        let auth = ctx.accounts.authority.key();
        if dig_identity_acc.authority == auth {
            msg!("Digital Identity Verified");
            Ok(())
        } else {
            return Err(DigitalIdentityErrorCode::DigitalIdentityDoesntMatchWithAuth.into());
        }
    }

    pub fn verify_proofs(ctx: Context<VerifyProofs>) -> Result<()> {
        let ref dig_proofs_acc = ctx.accounts.dig_proofs_acc;
        let ref dig_identity_acc = ctx.accounts.dig_identity_acc;
        let auth = ctx.accounts.authority.key();
        if dig_identity_acc.authority == auth
            && dig_proofs_acc.authority == auth
            && dig_proofs_acc.dig_identity_pubkey == dig_identity_acc.key()
        {
            msg!("Digital Identity Verified");
            Ok(())
        } else {
            return Err(DigitalIdentityErrorCode::DigitalProofDoesntMatchWithAuth.into());
        }
    }
}
