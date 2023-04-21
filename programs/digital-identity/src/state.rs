use anchor_lang::prelude::*;

#[account]
pub struct DigitalIdentity {
    pub name: String,
    pub authority: Pubkey,
    pub contact_number: String,
    pub dob: String,
    pub residence_address: String,
    pub pan_number: String,
    pub aadhar_number: String,
    pub passport_id: String,
    pub passport_attached: bool,
    pub aadhar_attached: bool,
    pub pan_attached: bool,
    pub pic_attached: bool,
}

#[account]
pub struct DigitalProofs {
    pub authority: Pubkey,
    pub dig_identity_pubkey: Pubkey,
    pub pan_upload: String,
    pub passport_upload: String,
    pub picture_upload: String,
    pub aadhar_upload: String,
}

impl DigitalIdentity {
    pub fn space() -> usize {
        return (8 + 32 + 7 * 32 + 4) as usize;
    }
}

impl DigitalProofs {
    pub fn space() -> usize {
        return (8 + 32 * 2 + 32 * 4) as usize;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DigitalIdentityParam {
    pub name: String,
    pub contact_number: String,
    pub dob: String,
    pub residence_address: String,
    pub pan_number: String,
    pub aadhar_number: String,
    pub passport_id: String,
    pub is_passport_attached: bool,
    pub is_aadhar_attached: bool,
    pub is_pan_attached: bool,
    pub is_pic_attached: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DigiatProofsParam {
    pub pan_upload: String,
    pub passport_upload: String,
    pub picture_upload: String,
    pub aadhar_upload: String,
}
