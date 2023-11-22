pub trait Encryptor {
    fn encrypt_card(ci: RawCardInfo) -> EncryptedCardInfo;
    
}