/// Trait that represents cipher algorithm
pub trait Cipher {
    /// Encrypts data with key
    /// # Parameters
    /// * `key` - encryption key
    /// * `data` - data to encrypt
    /// # Returns 
    /// Encrypted data
    fn encrypt(key: &Vec<u8>, data: &Vec<u8>) -> Vec<u8>;
    /// Decrypts data with key
    /// # Parameters
    /// * `key` - encryption key
    /// * `data` - data to decrypt
    /// # Returns 
    /// Decrypted data
    fn decrypt(key: &Vec<u8>, data: &Vec<u8>) -> Vec<u8>;
}