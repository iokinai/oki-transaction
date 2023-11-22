# Oki Transaction

The open-source protocol for bank transactions

### Protocol structure
Data sends as a plain JSON text structured like this:
```javascript
{
    sender: {
        number: aes_encrypted_card_number,
        expire: aes_encrypted_card_expiry_date_in_the_unix_timestamp_format,
        cvc: aes_encrypted_card_cvc_code
    },
    receiver: {
        number: aes_encrypted_card_number,
        expire: aes_encrypted_card_expiry_date_in_the_unix_timestamp_format,
        cvc: aes_encrypted_card_cvc_code
    },
    amount: amount,
    checksum: sha256_checksum
}

```

Here, `sender` and `receiver` represent the card data of sender and receiver, respectively. `amount` represent the amount of the transaction, `sha256_checksum` represent the checksum which is calculated by concatenation of hex-stringified `sender.aes_encrypted_card_number`, `receiver.aes_encrypted_card_number` and `amount`. You can find examples in the [examples](examples/) directory. The `sender` and `receiver` fields are of type byte array; `amount` is of type `float`, `sha256_checksum` is of type byte array. The encryption key for the `sender` and `receiver` fields is an SHA3-256 hash of a concatenation of sender's username and password and receiver's username and password. It looks like this: `"'sender-username''sender-password''receiver-username''receiver-password'"`. The fact that the key uses the data of both users, and not just the one whose data is encrypted, guarantees that during interception an attacker will not be able to replace the receiver's data with his own.