use argon2::password_hash::{Error, PasswordHash, SaltString, rand_core::RngCore};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub fn hash_password(password: &str) -> Result<String, Error> {
    // Generate a random salt
    let mut rng = ChaCha20Rng::from_entropy();
    let mut salt_bytes = [0u8; 32];
    rng.fill_bytes(&mut salt_bytes);

    let salt = SaltString::generate(&mut rng);

    // hash the password
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
