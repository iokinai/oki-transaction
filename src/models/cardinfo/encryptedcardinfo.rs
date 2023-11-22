use super::RawCardInfo;
use crate::models::encryptor::Encryptor;

pub struct EncryptedCardInfo {
    encrypted_number: Vec<u8>,
    encrypted_expire_date: Vec<u8>,
    encrtypted_cvc: Vec<u8>,
}

impl EncryptedCardInfo {
    pub fn create<T: Encryptor>(ci: RawCardInfo) {
        T::encrypt_card(ci)
    }
}