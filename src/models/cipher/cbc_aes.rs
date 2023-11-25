use serde::{Serialize, Deserialize};
use crypto::{blockmodes, buffer::{RefReadBuffer, RefWriteBuffer, WriteBuffer, ReadBuffer}};
use rand::Rng;

use super::Cipher;

#[derive(Serialize, Deserialize)]

/// Represents an AES encryption algorithm in CBC mode
pub struct CBCAES256;

impl Cipher for CBCAES256 {
    fn encrypt(key: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
        let mut encryptor = crypto::aes::cbc_encryptor(crypto::aes::KeySize::KeySize256, key, &Self::generate_random_iv(16), blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = RefReadBuffer::new(data);
        let mut buffer: [u8; 4096] = [0; 4096];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);

        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();

            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                crypto::buffer::BufferResult::BufferUnderflow => break,
                crypto::buffer::BufferResult::BufferOverflow => { },
            }
        };

        final_result
    }

    fn decrypt(key: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
        let mut decryptor = crypto::aes::cbc_decryptor(crypto::aes::KeySize::KeySize256, key, &Self::generate_random_iv(16), blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = RefReadBuffer::new(data);
        let mut buffer: [u8; 4096] = [0; 4096];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();

            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                crypto::buffer::BufferResult::BufferUnderflow => break,
                crypto::buffer::BufferResult::BufferOverflow => { },
            }
        };

        final_result
    }
}

impl CBCAES256 {
    /// Generated random Initialize Vector (IV)
    /// # Parameters
    /// * `size` - size of IV
    /// # Returns 
    /// Generated IV
    fn generate_random_iv(size: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();

        let mut iv = Vec::with_capacity(size);

        for _ in 0..size {
            iv.push(rng.gen());
        }

        iv
    }
}