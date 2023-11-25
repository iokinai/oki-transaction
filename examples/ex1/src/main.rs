use okitransaction::models::{OkiRequest, AccountInfo, RawCardInfo};
use sha2::{Sha256, Digest};
use chrono::DateTime;

fn main() {
    let mut hasher_s1 = Sha256::new();
    let mut hasher_s2 = Sha256::new();

    hasher_s1.update("ilovefurry");

    let sender_password_bytes = hasher_s1.finalize();

    hasher_s2.update("zxcursed");

    let receiver_password_bytes = hasher_s2.finalize();

    let sender = AccountInfo::new("nobody".to_string(), sender_password_bytes.to_vec(), RawCardInfo::new("4506 4405 3322 5050".to_string(), DateTime::from_timestamp(1917032400, 0).unwrap(), 304));
    let receiver = AccountInfo::new("okinai".to_string(), receiver_password_bytes.to_vec(), RawCardInfo::new("2200 0303 0404 0505".to_string(), DateTime::from_timestamp(1853960400, 0).unwrap(), 945));

    let transaction = OkiRequest::new(&sender, &receiver, 3000f64);
}
