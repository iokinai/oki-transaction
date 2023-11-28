use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Error;
use sha2::{Sha256, Digest};
use sha2::digest::{core_api::{CoreProxy, UpdateCore, FixedOutputCore, BufferKindUser}, HashMarker, block_buffer::Eager, crypto_common::BlockSizeUser, typenum::{IsLess, U256, NonZero, Le}};
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
    pub fn new<D>(sender: &AccountInfo, receiver: &AccountInfo, amount: f64) -> OkiRequest
        where
            D: CoreProxy,
            D::Core: Sync + HashMarker + UpdateCore + FixedOutputCore + BufferKindUser<BufferKind = Eager> + Default + Clone,
            <D::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
            Le<<D::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
     {
        let salt = okicrypto::generate_salt();
        let password = format!("{}{}{}{}", sender.name(), hex::encode(sender.password_hash()), receiver.name(), hex::encode(receiver.password_hash()));
        let key = okicrypto::generate_key::<D>(password, &salt);

        let encrypted_sender = sender.card().encrypt(&key.to_vec());
        let encrypted_receiver = receiver.card().encrypt(&key.to_vec());

        let mut sha256 = Sha256::new();
        let string_to_hash = format!("{}{}{}", sender.card().number(), receiver.card().number(), format!("{:.2}", amount));

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
    pub fn as_json(&self) -> Result<String, Error> {
        serde_json::to_string(self)
    }

    /// Returns `self.sender`
    pub fn sender(&self) -> &EncryptedCardInfo<CBCAES256> {
        &self.sender
    }

    /// Returns `self.receiver`
    pub fn receiver(&self) -> &EncryptedCardInfo<CBCAES256> {
        &self.receiver
    }

    /// Returns `self.amount`
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Returns `self.checksum`
    pub fn checksum(&self) -> [u8; 32] {
        self.checksum
    }

    /// Return `self.keysalt`
    pub fn keysalt(&self) -> [u8; 16] {
        self.keysalt
    }
}

impl Display for OkiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("[Sender: {}\nReceiver: {}\nAmount: {}\nChecksum: {}\nKeysalt: {}]", self.sender(), self.receiver(), self.amount(), hex::encode(self.checksum()), hex::encode(self.keysalt())))
    }
}