use bcrypt::{BcryptResult, DEFAULT_COST, hash, verify};
use rand::Rng;
use rand::distributions::Alphanumeric;

pub fn hash_password(password: String) -> String {
    return hash(password, DEFAULT_COST).unwrap();
}

pub fn verify_password(password: String, hashed: &str) -> BcryptResult<bool> {
    let test = hash_password(password.clone());
    return verify(password, hashed);
}

pub fn generate_random_string(length: i8) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
}