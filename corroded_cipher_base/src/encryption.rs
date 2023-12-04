// import necessary OpenSSL library crates
use openssl::symm::{Cipher, Crypter, Mode};
use openssl::error::ErrorStack;
use openssl::rand::rand_bytes;

use crate::encryption_params::{IV_LENGTH, AES_128_KEY_LENGTH, AES_192_KEY_LENGTH, AES_256_KEY_LENGTH};

pub fn generate_secure_iv(length: usize) -> Result<Vec<u8>, ErrorStack> {
    let mut iv = vec![0u8; length];
    rand_bytes(&mut iv)?;
    Ok(iv)
}

/// Accepts a byte slice of data to be encrypted, a key of length 16 to perform the encryption
/// with, and an initialization vector of length 16. Returns an Ok result containing the encrypted
/// data if all goes well, otherwise returns an error
pub fn encrypt_aes_128(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ensure that key and IV lengths are valid for AES-128-CBC
    if key.len() != AES_128_KEY_LENGTH || iv.len() != IV_LENGTH {
        return Err(ErrorStack::get());
    }
    
    // choose the AES-128-CBC cipher.
    let cipher = Cipher::aes_128_cbc();
    
    // Create a Crypter in encryption mode
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;

    // Encrypt and finalize data
    let mut encrypted = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(data, &mut encrypted)?;
    let rest = crypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest); // remove any padding bytes
    
    // append the iv to the beginning of the result
    let mut result = Vec::with_capacity(iv.len() + encrypted.len());
    result.extend_from_slice(iv);
    result.extend_from_slice(&encrypted);

    Ok(result)
}

/// Accepts a byte slice of data to be encrypted, a key of length 24 to perform the encryption
/// with, and an initialization vector of length 16. Returns an Ok result containing the encrypted
/// data if all goes well, otherwise returns an error
pub fn encrypt_aes_192(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ensure that key and IV lengths are valid for AES-192-CBC
    if key.len() != AES_192_KEY_LENGTH || iv.len() != IV_LENGTH {
        return Err(ErrorStack::get());
    }
    
    // choose the AES-192-CBC cipher.
    let cipher = Cipher::aes_192_cbc();
    
    // Create a Crypter in encryption mode
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;

    // Encrypt and finalize data
    let mut encrypted = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(data, &mut encrypted)?;
    let rest = crypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest); // remove any padding bytes
   
    // append the iv to the beginning of the result
    let mut result = Vec::with_capacity(iv.len() + encrypted.len());
    result.extend_from_slice(iv);
    result.extend_from_slice(&encrypted);

    Ok(result)
}

/// Accepts a byte slice of data to be encrypted, a key of length 32 to perform the encryption
/// with, and an initialization vector of length 16. Returns an Ok result containing the encrypted
/// data if all goes well, otherwise returns an error
pub fn encrypt_aes_256(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ensure that key and IV lengths are valid for AES-256-CBC
    if key.len() != AES_256_KEY_LENGTH || iv.len() != IV_LENGTH {
        return Err(ErrorStack::get());
    }
    
    // choose the AES-256-CBC cipher.
    let cipher = Cipher::aes_256_cbc();
    
    // Create a Crypter in encryption mode
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;

    // Encrypt and finalize data
    let mut encrypted = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(data, &mut encrypted)?;
    let rest = crypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest); // remove any padding bytes
    
    // append the iv to the beginning of the result
    let mut result = Vec::with_capacity(iv.len() + encrypted.len());
    result.extend_from_slice(iv);
    result.extend_from_slice(&encrypted);

    Ok(result)
}
