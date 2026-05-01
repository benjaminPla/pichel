use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash as Argon2PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
use async_trait::async_trait;
use tokio::task;

use crate::domain::user::{
    ports::hasher::{Hasher, HasherError},
    value_objects::{password_hash::PasswordHash, password_raw::PasswordRaw},
};

pub struct Argon2Hasher {
    argon2: Argon2<'static>,
}

impl Argon2Hasher {
    const M_COST: u32 = 12_288;
    const T_COST: u32 = 3;
    const P_COST: u32 = 1;

    pub fn new() -> Self {
        let params = Params::new(Self::M_COST, Self::T_COST, Self::P_COST, None).expect("invalid argon2 params");
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        Self { argon2 }
    }
}

#[async_trait]
impl Hasher for Argon2Hasher {
    async fn hash(&self, raw: &PasswordRaw) -> Result<PasswordHash, HasherError> {
        let argon2 = self.argon2.clone();
        let bytes  = raw.value().as_bytes().to_vec();
        task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let hash = argon2
                .hash_password(&bytes, &salt)
                .map_err(|_| HasherError::Hash)?
                .to_string();
            Ok(PasswordHash::new(hash))
        })
        .await
        .map_err(|_| HasherError::Hash)?
    }

    async fn verify(&self, raw: &PasswordRaw, hash: &PasswordHash) -> Result<bool, HasherError> {
        let argon2   = self.argon2.clone();
        let bytes    = raw.value().as_bytes().to_vec();
        let hash_str = hash.value().to_string();
        task::spawn_blocking(move || {
            let parsed = Argon2PasswordHash::new(&hash_str).map_err(|_| HasherError::Verify)?;
            Ok(argon2.verify_password(&bytes, &parsed).is_ok())
        })
        .await
        .map_err(|_| HasherError::Verify)?
    }
}
