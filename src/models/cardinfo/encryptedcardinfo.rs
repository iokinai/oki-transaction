use chrono::DateTime;
use serde::{Serialize, Deserialize};

use super::RawCardInfo;
use crate::models::Cipher;
use crate::models::FromBytesError;

/// Represents encrypted with `T` algorhytm card info
/// # Parameters
/// * `ci` - `&RawCardInfo` which to encrypt
/// * `key` - Encryption key of type `&Vec<u8>`
/// 
/// # Returns
/// `EncryptedCardInfo<T>` where `T` is an encryption algorhytm
#[derive(Serialize, Deserialize)]
pub struct EncryptedCardInfo<T> {
    enc: Option<T>,
    number: Vec<u8>,
    expire: Vec<u8>,
    cvc: Vec<u8>,
}

impl<T: Cipher> EncryptedCardInfo<T> {
    /// Creates new instance of `EncryptedCardInfo<T>`
    /// # Parameters 
    /// * `ci` - user's raw card info
    /// * `key` - encryption key
    pub fn create(ci: &RawCardInfo, key: &Vec<u8>) -> EncryptedCardInfo<T> {
        EncryptedCardInfo {
            enc: None,
            number: T::encrypt(key, &ci.number().as_bytes().to_vec()),
            expire: T::encrypt(key, &ci.expire().timestamp().to_le_bytes().to_vec()),
            cvc: T::encrypt(key, &ci.cvc().to_le_bytes().to_vec()),
        }
    }

    /// Returns self.number
    pub fn number(&self) -> &Vec<u8> {
        &self.number
    }

    /// Returns self.expire
    pub fn expire(&self) -> &Vec<u8> {
        &self.expire
    }

    /// Returns self.cvc
    pub fn cvc(&self) -> &Vec<u8> {
        &self.cvc
    }

    /// Decrypts this `EncryptedCardInfo<T>` into `RawCardInfo`
    /// # Parameters 
    /// * `key` - encryption key of type `&Vec<u8>`
    pub fn decrypt(&self, key: &Vec<u8>) -> Result<RawCardInfo, FromBytesError> {
        let secs_bytes = T::decrypt(key, self.expire());

        if secs_bytes.len() != 8  {
            return Result::Err(FromBytesError {  })
        }

        let cvc_bytes = T::decrypt(key, self.cvc());

        if cvc_bytes.len() != 8 {
            return Result::Err(FromBytesError{})
        }

        let secs = i64::from_le_bytes(secs_bytes.try_into().unwrap());
        let cvc = u16::from_le_bytes(cvc_bytes.try_into().unwrap());
        let from_utf8 = String::from_utf8(T::decrypt(&key, self.number()));

        Ok(RawCardInfo::new(from_utf8.unwrap(), DateTime::from_timestamp(secs, 0).unwrap(), cvc))
    }
}