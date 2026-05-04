mod errors;

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash as Argon2PasswordHash, PasswordHasher,
        PasswordVerifier, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
use async_trait::async_trait;
use crate::domain::{
    hasher::{HasherService, HasherServiceError},
    user::value_objects::{password_hash::PasswordHash, password_raw::PasswordRaw},
};
use tokio::task;

pub struct Argon2HasherService {
    argon2: Argon2<'static>,
}

impl Argon2HasherService {
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
impl HasherService for Argon2HasherService {
    async fn hash(&self, raw: &PasswordRaw) -> Result<PasswordHash, HasherServiceError> {
        let argon2 = self.argon2.clone();
        let bytes  = raw.value().as_bytes().to_vec();
        task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let hash = argon2
                .hash_password(&bytes, &salt)?
                .to_string();
            Ok(PasswordHash::new(hash))
        })
        .await?
    }

    async fn verify(&self, raw: &PasswordRaw, hash: &PasswordHash) -> Result<bool, HasherServiceError> {
        let argon2   = self.argon2.clone();
        let bytes    = raw.value().as_bytes().to_vec();
        let hash_str = hash.value().to_string();
        task::spawn_blocking(move || -> Result<bool, HasherServiceError> {
            let parsed = Argon2PasswordHash::new(&hash_str)?;
            argon2.verify_password(&bytes, &parsed)?;
            Ok(true)
        })
        .await?
    }
}
