# Oki Transaction

Oki Transaction is an open-source protocol for bank transactions that allows users to send and receive money securely and efficiently. The protocol uses JSON format to send data and encryption algorithms to protect sensitive information.


### Data Format
Data sends as a plain JSON text structured like this:
```javascript
{
    sender: {
        enc: null
        number: card_number,
        expire: aes_encrypted_card_expiry_date_in_the_unix_timestamp_format,
        cvc: aes_encrypted_card_cvc_code
    },
    receiver: {
        enc: null
        number: card_number,
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

#### Number
* `number` should be card number of type utf-8 string (as one word). For example, `"2400 3500 2200 5566"` is wrong whereas `"2400350022005566" is correct`

#### Encrypted fields:
* `enc` should be `null`
* `expire`, `cvc` should be of type byte array

#### Encryption algorithm:
* We use AES algorithm with CBC mode to encrypt data.

#### Raw data to encrypt:
* Raw `expire` should be of type `int64`, Unix timestamp format (in seconds), bytes should be in little-endian order\*
* Raw `cvc` should be of type `unsigned int16`, bytes should be in little-endian order\*

#### Encryption key:
Encryption key should be a PBKDF2 key with following parameters:
* `PRF` - HMAC with SHA-256 hash function
* `Password` - concatenation of sender's username and password lower-cased hex-stringified hash in algorithm you use to store passwords and receiver's username and password hash that looks like `"'sender-username''sender-password-hash''receiver-username''receiver-password-hash'"`. For example, if we have: 
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
then `Password` will look like `"senderc8eb6be7da27a445473bc50d1bbf60d91e06b1332156ffdd252d87270bf351bfreceiverb23a27949d63ec1acc7f39912237881cf86b8f3f59d1269b2cdb6dfcf879d6bb"`
The fact that the key uses the data of both users, and not just the one whose data is encrypted, guarantees that during interception an attacker will not be able to replace the receiver's data with his own.
* `Salt` - random 16 bytes (128 bits), which are transferred in `keysalt` field
* `c` - 25_000
* `dkLen` - 16 bytes (128 bits)

For example PBKDF2 with following parameters:

* `Password` - `senderc8eb6be7da27a445473bc50d1bbf60d91e06b1332156ffdd252d87270bf351bfreceiverb23a27949d63ec1acc7f39912237881cf86b8f3f59d1269b2cdb6dfcf879d6bb`
* `Salt` - `ppt8FWth+EJ2ajP8`
* `algorithm` - `SHA3-256`

will generate hex string `fa2e7780a138c523ddc4dabc69d71c01`.

In Rust it can be described with following code:

```rust
let mut key = [0u8; 16];
pbkdf2_hmac::<Sha3256>(b"senderc8eb6be7da27a445473bc50d1bbf60d91e06b1332156ffdd252d87270bf351bfreceiverb23a27949d63ec1acc7f39912237881cf86b8f3f59d1269b2cdb6dfcf879d6bb", b"ppt8FWth+EJ2ajP8", 25_000, &mut key);
println!("{:x?}", key);
```

This code will print:

```rust
[fa, 2e, 77, 80, a1, 38, c5, 23, dd, c4, da, bc, 69, d7, 1c, 1]
```

### Other fields

* `amount` represent the amount of the transaction, it is of type `float64` with two digits after comma.
* `checksum` represent the checksum which is calculated with SHA-256 by concatenation of lowercased hex-stringified `sender.card_number`, `receiver.card_number` and rounded to 2 digits after comma `amount`, it is of type byte array. 

For example, if you have:

`sender.card_number` = `0000000000000000`,
`receiver.card_number` = `1111111111111111`,
`amount` = `100.00`,

then you will hash the `00000000000000001111111111111111100.00` string and get `9b1cc5a9ed9928d46d44064701ae57f125e634b4877985eb7ffcdad1a9dde674`.

* `keysalt` is 16 bytes array that represents a salt used in key generation. See [Encryption Key](#encryption-key) for more information.

You can find examples in the [examples](examples/) directory. 

## References
* [JSON](https://www.w3schools.com/whatis/whatis_json.asp) - JavaScript Object Notation, a lightweight data-interchange format.
* [AES](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) - Advanced Encryption Standard, a symmetric-key encryption algorithm.
* [PBKDF2](https://en.wikipedia.org/wiki/PBKDF2) - Password-Based Key Derivation Function 2, a key-stretching algorithm.
* [SHA-256](https://en.wikipedia.org/wiki/SHA-2) - Secure Hash Algorithm 256, a cryptographic hash function.

\* While encrypting data you'll have to convert it into bytes. With numbers, you have to decide whether little-endian or big-endian order to use. In this protocol you should choose little-endian
