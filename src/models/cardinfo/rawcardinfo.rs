use chrono::{DateTime, Utc};

use crate::models::Cipher;

use super::EncryptedCardInfo;

/// Card info without encryption
pub struct RawCardInfo {
    number: String,
    expires: DateTime<Utc>,
    cvc: u16,
}

impl RawCardInfo {
    /// Creates new `RawCardInfo`.
    /// # Parameters
    /// * `number` - card number of type `String` (without spaces)
    #[inline]
    pub fn new(number: String, expires: DateTime<Utc>, cvc: u16) -> RawCardInfo {
        RawCardInfo { number, expires, cvc }
    }

    pub fn get_number(&self) -> &String {
        &self.number
    }

    pub fn get_expires(&self) -> &DateTime<Utc> {
        &self.expires
    }

    pub fn get_cvc(&self) -> u16 {
        self.cvc
    }

    pub fn encrypt<T: Cipher>(&self, key: &Vec<u8>) -> EncryptedCardInfo<T> {
        EncryptedCardInfo::<T>::create(self, key)
    }
}