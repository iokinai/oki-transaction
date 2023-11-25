use pbkdf2::pbkdf2_hmac;
use rand::{thread_rng, Rng};
use sha2::Sha256;

/// Generates random 16 bytes salt 
pub fn generate_salt() -> [u8; 16] {
    let mut rng = thread_rng();
    rng.gen()
}

/// Generates key with PBKDF2 based on password and salt (hash: SHA-256, rounds: 25_000)
pub fn generate_key(password: String, salt: &[u8; 16]) -> [u8; 16] {
    let mut result = [0u8; 16];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 25_000, &mut result);
    result
}