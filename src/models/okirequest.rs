use serde::{Deserialize, Serialize, de::Error};
use sha2::{Sha256, Digest};
use super::{EncryptedCardInfo, CBCAES256, okicrypto, AccountInfo};

/// Represents request 
#[derive(Serialize, Deserialize)]
pub struct OkiRequest {
    sender: EncryptedCardInfo<CBCAES256>,
    receiver: EncryptedCardInfo<CBCAES256>,
    amount: f64,
    checksum: [u8; 32],
    keysalt: [u8; 16],
}

impl OkiRequest {
    /// Creates new `OkiRequest`
    /// # Parameters:
    /// * `sender` - sender's account info
    /// * `receiver` - receiver's account info
    /// * `amount` - transaction's amount
    pub fn new(sender: &AccountInfo, receiver: &AccountInfo, amount: f64) -> OkiRequest {
        let salt = okicrypto::generate_salt();
        let password = format!("{}{}{}{}", sender.get_name(), hex::encode(sender.get_password_hash()), receiver.get_name(), hex::encode(receiver.get_password_hash()));
        let key = okicrypto::generate_key(password, &salt);

        let encrypted_sender = sender.get_card().encrypt(&key.to_vec());
        let encrypted_receiver = receiver.get_card().encrypt(&key.to_vec());

        let mut sha256 = Sha256::new();
        let string_to_hash = format!("{}{}{}", sender.get_card().number(), receiver.get_card().number(), format!("{:.2}", amount));

        sha256.update(string_to_hash);

        let checksum = sha256.finalize().into();


        OkiRequest { sender: encrypted_sender, receiver: encrypted_receiver, amount, checksum, keysalt: salt }
    }

    /// Creates new `OkiRequest` with existing data
    /// # Parameters
    /// * `sender` - `EncryptedCardInfo<CBCAES256>` of sender
    /// * `receiver` - `EncryptedCardInfo<CBCAES256>` of receiver
    /// * `am` - amount of transation
    /// * `sender_card_number` - raw sender's card number
    /// * `receiver_card_number` - raw receiver's card number
    /// * `key_salt` - salt when generating an encryption key
    pub fn new_existing(sender: EncryptedCardInfo<CBCAES256>, receiver: EncryptedCardInfo<CBCAES256>, am: f64, sender_card_number: String, receiver_card_number: String, keysalt: [u8; 16]) -> OkiRequest {
        let amount = (am * 100.0).round() / 100.0;
        let mut sha256 = Sha256::new();

        let string_to_hash = format!("{}{}{}", sender_card_number, receiver_card_number, format!("{:.2}", am));

        sha256.update(string_to_hash);

        let checksum = sha256.finalize().into();

        OkiRequest { sender, receiver, amount, checksum, keysalt }
    }


    /// Serializes `self` and returns JSON string
    pub fn as_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    /// Returns self.sender
    pub fn sender(&self) -> &EncryptedCardInfo<CBCAES256> {
        &self.sender
    }

    /// Return self.receiver
    pub fn receiver(&self) -> &EncryptedCardInfo<CBCAES256> {
        &self.receiver
    }

    /// Returns self.amount
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Returns self.checksum
    pub fn checksum(&self) -> [u8; 32] {
        self.checksum
    }

    /// Return self.keysalt
    pub fn keysalt(&self) -> [u8; 16] {
        self.keysalt
    }
}