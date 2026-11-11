use jsonwebtoken::signature::rand_core::impls::fill_bytes_via_next;
use rand::fill;
use rand_core::TryRng;

pub fn generate_random_hash(len: usize) -> String {
    let mut bytes = vec![0u8; len];

    rand::fill(&mut bytes);

    hex::encode(bytes)
}