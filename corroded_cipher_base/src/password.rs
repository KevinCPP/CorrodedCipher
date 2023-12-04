
use openssl::pkcs5::pbkdf2_hmac;
use openssl::hash::MessageDigest;
use openssl::error::ErrorStack;
use openssl::rand::rand_bytes;

/// This function will generate random salt to be used. Salt is random data that is used as
/// additional input to a one-way function that hashes data. Salts are used to improve the
/// security of passwords in storage. It will return a salt of `length` bytes.
fn generate_random_salt(length: usize) -> Result<Vec<u8>, ErrorStack> {
    // vector that will store the random salt data 
    let mut salt = vec![0u8; length];

    // use rand_bytes function from openssl rand to generate random bytes
    // to be used in the salt. This will populate the vector we just created
    rand_bytes(&mut salt)?;

    // if all went well, return an Ok result containing the random salt
    Ok(salt)
}

/// This function accepts a password and a key length, and returns a key of length key_length
/// which is derived from the password. key_length should be 16 for AES128, 24 for AES192, and 32
/// for AES256.
pub fn derive_key_from_password(password: &str, key_length: usize) -> Result<Vec<u8>, ErrorStack> {
    // define the salt size (16 is commonly used)
    //let salt_size = 16;
    // Generate the salt, handling the result properly with error propagation
    let salt = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e];
        
    // define the number of iterations to use (10000 is usually the minimum used in real world
    // applications, but usually more is used. For this purpose, I will use 10000)
    let iterations = 10_000;
    
    // This will store the key where the password is derived
    let mut key = vec![0u8; key_length];
    
    // derive a key of length `key_length` from the password
    pbkdf2_hmac(
        password.as_bytes(),        // convert the password string into a slice of bytes
        &salt[..],                  // pass the salt as a slice of bytes
        iterations,                 // number of iterations to use for pbkdf2
        MessageDigest::sha256(),    // Use sha256 in the password-to-key-derivation
        &mut key,                   // store the result in the key vector
    )?;
    
    // if all went well, return an Ok result containing the password derived key
    Ok(key)
}
