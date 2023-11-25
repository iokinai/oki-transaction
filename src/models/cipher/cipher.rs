pub trait Cipher {
    fn encrypt(key: &Vec<u8>, data: &Vec<u8>) -> Vec<u8>;
    fn decrypt(key: &Vec<u8>, data: &Vec<u8>) -> Vec<u8>;
}