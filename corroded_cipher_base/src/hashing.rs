use openssl::hash::{hash, MessageDigest};

/// This function will accept input bytes and return the sha256 hash
pub fn sha256_hash(data: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    hash(MessageDigest::sha256(), &data).map(|hash| hash.to_vec())
}
