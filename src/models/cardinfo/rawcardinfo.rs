use chrono::{Date, Utc};

pub struct RawCardInfo {
    number: String,
    expires: Date<Utc>,
    cvc: u16,
}

impl RawCardInfo {
    pub fn new(number: String, expires: Date<Utc>, cvc: u16) -> RawCardInfo {
        RawCardInfo { number, expires, cvc }
    }

    pub fn get_number(&self) -> String {
        self.number
    }

    pub fn get_expires(&self) -> Date<Utc> {
        self.expires
    }

    pub fn get_cvc(&self) -> u16 {
        self.cvc
    }

    pub fn encrypt(acc: AccountInfo) -> EncryptedCardInfo {

    }
}