use crate::{Error, Result};
use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher,
    PasswordVerifier,
};
use rand_core::OsRng;

#[derive(Debug, Clone)]
pub struct Password(pub String);

impl Password {
    /// 低强度校验通用密码(用户决定密码强度)
    pub fn parse<T: Into<String>>(val: T) -> Result<Self> {
        let pwd = val.into();
        let len = pwd.len();

        if len == 0 || len > 16 {
            return Err(Error::bad_format("password")
                .set_message("password length must be between 1 and 16"));
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(pwd.as_bytes(), &salt)
            .map_err(|e| {
                Error::internal("pwd", "unknown").set_message(e.to_string())
            })?
            .to_string();

        Ok(Self(hash))
    }

    pub fn verify(&self, pwd: &[u8]) -> Result<bool> {
        let parsed_hash = PasswordHash::new(&self.0).map_err(|e| {
            Error::internal("pwd", "unknown").set_message(e.to_string())
        })?;

        let argon2 = Argon2::default();

        Ok(argon2.verify_password(pwd, &parsed_hash).is_ok())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
