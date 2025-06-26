use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString, rand_core::OsRng,
    },
};

pub struct PasswordHasher {
    algorithm: Argon2<'static>,
}

impl PasswordHasher {
    /// Creates a new PasswordHasher with default Argon2 config
    pub fn new() -> Self {
        Self {
            algorithm: Argon2::default(),
        }
    }

    /// Hashes a plain password and returns the hash string
    pub fn hash_password(&self, plain: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed = self.algorithm.hash_password(plain.as_bytes(), &salt)?;
        Ok(hashed.to_string())
    }

    /// Verifies a plain password against a hashed password
    pub fn verify_password(&self, plain: &str, hashed: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(hashed) {
            self.algorithm
                .verify_password(plain.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }
}
