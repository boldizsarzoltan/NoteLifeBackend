use bcrypt::{hash, verify, DEFAULT_COST, BcryptResult};

pub fn hash_password(password: String) -> String {
    return hash(password, DEFAULT_COST).unwrap();
}

pub fn verify_password(password: String, hashed: &str) -> BcryptResult<bool> {
    return verify(password, hashed);
}