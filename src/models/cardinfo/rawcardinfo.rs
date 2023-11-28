use chrono::{DateTime, Utc};

use crate::models::Cipher;

use super::EncryptedCardInfo;

/// Card info without encryption
pub struct RawCardInfo {
    number: String,
    expire: DateTime<Utc>,
    cvc: u16,
}

impl RawCardInfo {
    /// Creates new `RawCardInfo`.
    /// # Parameters
    /// * `number` - card number of type `String` (without spaces)
    pub fn new(number: String, expires: DateTime<Utc>, cvc: u16) -> RawCardInfo {
        let clear_number = number.replace(" ", "");
        RawCardInfo { number: clear_number, expire: expires, cvc }
    }

    /// Returns `self.number`
    pub fn number(&self) -> &String {
        &self.number
    }

    /// Returns `self.expire`
    pub fn expire(&self) -> &DateTime<Utc> {
        &self.expire
    }

    /// Returns `self.cvc`
    pub fn cvc(&self) -> u16 {
        self.cvc
    }

    /// Creates new instance of `EncryptedCardInfo<T>` based on key
    /// # Parameters
    /// * `key` - encryption key
    pub fn encrypt<T: Cipher>(&self, key: &Vec<u8>) -> EncryptedCardInfo<T> {
        EncryptedCardInfo::<T>::create(self, key)
    }
}