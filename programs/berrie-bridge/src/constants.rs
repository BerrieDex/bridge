use anchor_lang::{constant, prelude::Pubkey, pubkey};

#[constant]
pub const ADMIN_PUBKEY: Pubkey = pubkey!("athdoSSvTJoAxt9DY7cFrbGynu7Bv1MQUUB8NSstAD3");

pub mod seeds {
    pub const LOCK_TOKEN_SEED: &[u8] = b"lock_token";
    pub const LOCK_SEED: &[u8] = b"lock";
    pub const METADATA_SEED: &[u8] = b"metadata";
}
