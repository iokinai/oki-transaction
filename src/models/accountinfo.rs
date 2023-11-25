use super::RawCardInfo;

/// Represents user's account information
pub struct AccountInfo {
    name: String,
    password_hash: Vec<u8>,
    card: RawCardInfo
}
impl AccountInfo {
    /// Creates new instance of AccountInfo
    /// # Parameters
    /// * `name` - user's name
    /// * `password_hash` - user's password hash
    /// * `card` - user's card info
    pub fn new(name: String, password_hash: Vec<u8>, card: RawCardInfo) -> AccountInfo {
        AccountInfo { name, password_hash, card }
    }

    /// Returns self.name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns self.password_hash
    pub fn get_password_hash(&self) -> &Vec<u8> {
        &self.password_hash
    }

    /// Returns self.card
    pub fn get_card(&self) -> &RawCardInfo {
        &self.card
    }
}