use serde::{Deserialize, Serialize}
use super::{EncryptedCardInfo, CBCAES256};

#[derive(Serialize, Deserialize)]
pub struct OkiRequest {
    sender: EncryptedCardInfo<CBCAES256>
}