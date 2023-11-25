use serde::{Deserialize, Serialize}
use super::{EncryptedCardInfo, CBCAES256};

#[derive(Serialize, Deserialize)]
pub struct OkiRequest {
    sender: EncryptedCardInfo<CBCAES256>,
    receiver: EncryptedCardInfo<CBCAES256>,
    amount: f64,
    checksum: [u8; 32],
    keysalt: [u8; 16],
}