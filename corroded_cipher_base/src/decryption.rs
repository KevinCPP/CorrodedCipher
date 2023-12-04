// import necessary OpenSSL library crates
use openssl::symm::{Cipher, Crypter, Mode};
use openssl::error::ErrorStack;

use crate::encryption_params::{IV_LENGTH, AES_128_KEY_LENGTH, AES_192_KEY_LENGTH, AES_256_KEY_LENGTH};

/// Accepts a byte slice of encrypted data, where the first IV_LENGTH bytes are the IV,
/// and a key of length AES_128_KEY_LENGTH to perform the decryption with.
/// Returns an Ok result containing the decrypted data if all goes well, otherwise returns an error.
pub fn decrypt_aes_128(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ensure that key length is valid for AES-128-CBC
    if key.len() != AES_128_KEY_LENGTH {
        return Err(ErrorStack::get());
    }

    // ensure that the encrypted data has at least the length of the IV
    if encrypted_data.len() < IV_LENGTH {
        return Err(ErrorStack::get());
    }

    // split the IV from the encrypted data
    let (iv, encrypted_data) = encrypted_data.split_at(IV_LENGTH);

    // choose the AES-128-CBC cipher
    let cipher = Cipher::aes_128_cbc();

    // Create a Crypter in decryption mode
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;

    // Decrypt and finalize data
    let mut decrypted = vec![0; encrypted_data.len() + cipher.block_size()];
    let count = crypter.update(encrypted_data, &mut decrypted)?;
    let rest = crypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest); // remove any padding bytes

    Ok(decrypted)
}

/// Accepts a byte slice of encrypted data, where the first IV_LENGTH bytes are the IV,
/// and a key of length AES_192_KEY_LENGTH to perform the decryption with.
/// Returns an Ok result containing the decrypted data if all goes well, otherwise returns an error.
pub fn decrypt_aes_192(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ensure that key length is valid for AES-192-CBC
    if key.len() != AES_192_KEY_LENGTH {
        return Err(ErrorStack::get());
    }

    // ensure that the encrypted data has at least the length of the IV
    if encrypted_data.len() < IV_LENGTH {
        return Err(ErrorStack::get());
    }

    // split the IV from the encrypted data
    let (iv, encrypted_data) = encrypted_data.split_at(IV_LENGTH);

    // choose the AES-192-CBC cipher
    let cipher = Cipher::aes_192_cbc();

    // Create a Crypter in decryption mode
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;

    // Decrypt and finalize data
    let mut decrypted = vec![0; encrypted_data.len() + cipher.block_size()];
    let count = crypter.update(encrypted_data, &mut decrypted)?;
    let rest = crypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest); // remove any padding bytes

    Ok(decrypted)
}

/// Accepts a byte slice of encrypted data, where the first IV_LENGTH bytes are the IV,
/// and a key of length AES_256_KEY_LENGTH to perform the decryption with.
/// Returns an Ok result containing the decrypted data if all goes well, otherwise returns an error.
pub fn decrypt_aes_256(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ensure that key length is valid for AES-256-CBC
    if key.len() != AES_256_KEY_LENGTH {
        return Err(ErrorStack::get());
    }

    // ensure that the encrypted data has at least the length of the IV
    if encrypted_data.len() < IV_LENGTH {
        return Err(ErrorStack::get());
    }

    // split the IV from the encrypted data
    let (iv, encrypted_data) = encrypted_data.split_at(IV_LENGTH);

    // choose the AES-256-CBC cipher
    let cipher = Cipher::aes_256_cbc();

    // Create a Crypter in decryption mode
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;

    // Decrypt and finalize data
    let mut decrypted = vec![0; encrypted_data.len() + cipher.block_size()];
    let count = crypter.update(encrypted_data, &mut decrypted)?;
    let rest = crypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest); // remove any padding bytes

    Ok(decrypted)
}

