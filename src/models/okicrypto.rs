use pbkdf2::pbkdf2_hmac;
use rand::{thread_rng, Rng};
use sha2::digest::{core_api::{CoreProxy, UpdateCore, FixedOutputCore, BufferKindUser}, HashMarker, block_buffer::Eager, crypto_common::BlockSizeUser, typenum::{IsLess, U256, NonZero, Le}};

/// Generates random 16 bytes salt 
pub fn generate_salt() -> [u8; 16] {
    let mut rng = thread_rng();
    rng.gen()
}

/// Generates key with PBKDF2 based on password and salt (hash: SHA-256, rounds: 25_000)
pub fn generate_key<D>(password: String, salt: &[u8; 16]) -> [u8; 16] 
            where
                D: CoreProxy,
                D::Core: Sync + HashMarker + UpdateCore + FixedOutputCore + BufferKindUser<BufferKind = Eager> + Default + Clone,
                <D::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
                Le<<D::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    let mut result = [0u8; 16];
    pbkdf2_hmac::<D>(password.as_bytes(), salt, 25_000, &mut result);
    result
}