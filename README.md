# Oki Transaction

Oki Transaction is an open-source protocol for bank transactions that allows users to send and receive money securely and efficiently. The protocol uses JSON format to send data and encryption algorithms to protect sensitive information.


### Data Format
Data sends as a plain JSON text structured like this:
```javascript
{
    sender: {
        enc: null
        number: aes_encrypted_card_number,
        expire: aes_encrypted_card_expiry_date_in_the_unix_timestamp_format,
        cvc: aes_encrypted_card_cvc_code
    },
    receiver: {
        enc: null
        number: aes_encrypted_card_number,
        expire: aes_encrypted_card_expiry_date_in_the_unix_timestamp_format,
        cvc: aes_encrypted_card_cvc_code
    },
    amount: amount,
    checksum: sha256_checksum,
    keysalt: key_salt,
}

```

Here, 
* `sender` and `receiver` represent the card data of sender and receiver, respectively. They have the following subfields:

#### Encrypted fields:
* `enc` should be `null`
* `number`, `expire`, `cvc` should be of type byte array

#### Encryption algorithm:
* We use AES algorithm with CBC mode to encrypt data.

#### Raw data to encrypt:
* Raw `number` should be card number of type utf-8 string (as one word). For example, `"2400 3500 2200 5566"` is wrong whereas `"2400350022005566" is correct`
* Raw `expire` should be of type `int64`, Unix timestamp format (in seconds), bytes should be in little-endian order\*
* Raw `cvc` should be of type `unsigned int16`, bytes should be in little-endian order\*

#### Encryption key:
Encryption key should be a PBKDF2 key with following parameters:
* `PRF` - HMAC with SHA-256 hash function
* `Password` - concatenation of sender's username and password and receiver's username and password that looks like `"'sender-username''sender-password''receiver-username''receiver-password'"`. For example, if we have: 
```javascript
sender: {
    username: "sender",
    password: "password1"
},

receiver: {
    username: "receiver",
    password: "password1"
}
```
then `Password` will look like `"senderpassword1receiverpassword2"`
The fact that the key uses the data of both users, and not just the one whose data is encrypted, guarantees that during interception an attacker will not be able to replace the receiver's data with his own.
* `Salt` - random 16 bytes (128 bits), which are transferred in `keysalt` field
* `c` - 25_000
* `dkLen` - 16 bytes (128 bits)

For example PBKDF2 with following parameters:

* `Password` - `senderpassword1receiverpassword2`
* `Salt` - `ppt8FWth+EJ2ajP8`

will generate hex string `fb63d980ed9b1da1e257b41c699a3c92`.

In Rust it can be described with following code:

```rust
let mut key = [0u8; 16];
pbkdf2_hmac::<Sha256>(b"senderpassword1receiverpassword2", b"ppt8FWth+EJ2ajP8", 25_000, &mut key);
println!("{:x?}", key);
```

This code will print:

```rust
[fb, 63, d9, 80, ed, 9b, 1d, a1, e2, 57, b4, 1c, 69, 9a, 3c, 92]
```

### Other fields

* `amount` represent the amount of the transaction, it is of type `float64` with two digits after comma.
* `sha256_checksum` represent the checksum which is calculated with SHA-256 by concatenation of lowercased hex-stringified `sender.aes_encrypted_card_number`, `receiver.aes_encrypted_card_number` and `amount`, it is of type byte array.
* `keysalt` is 16 bytes array that represents a salt used in key generation. See [Encryption Key](#encryption-key) for more information.

You can find examples in the [examples](examples/) directory. 

## References
* [JSON](https://www.w3schools.com/whatis/whatis_json.asp) - JavaScript Object Notation, a lightweight data-interchange format.
* [AES](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) - Advanced Encryption Standard, a symmetric-key encryption algorithm.
* [PBKDF2](https://en.wikipedia.org/wiki/PBKDF2) - Password-Based Key Derivation Function 2, a key-stretching algorithm.
* [SHA-256](https://en.wikipedia.org/wiki/SHA-2) - Secure Hash Algorithm 256, a cryptographic hash function.

\* While encrypting data you'll have to convert it into bytes. With numbers, you have to decide whether little-endian or big-endian order to use. In this protocol you should choose little-endian