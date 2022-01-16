use hmac::{Hmac, Mac};
use rand::{distributions::Alphanumeric, Rng};
use sha2::Sha256;
use std::time::SystemTime;

use super::Algorithm;

#[derive(Debug, Clone)]
pub struct Signature {
    pub id: String,
    // #[cfg(feature = "hmac")]
    pub secret: String,

    // #[cfg(feature = "hmac")]
    pub algorithm: Algorithm,

    // #[cfg(feature = "hmac")]
    pub action: String,
}

impl Signature {
    pub fn nonce(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }
    pub fn timestamp(&self) -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    fn encrypt(&self, data: &str) -> String {
        match self.algorithm {
            Algorithm::HmacSha256 => {
                type HmacSha256 = Hmac<Sha256>;

                let mut hmac =
                    HmacSha256::new_from_slice(self.secret.as_bytes())
                        .expect("HMAC can take key of any size");

                hmac.update(data.as_bytes());

                hex::encode(hmac.finalize().into_bytes())
            }
        }
    }

    // pub fn generate<T>(&self, timestamp: &T) -> String
    // where
    //     T: fmt::Display,

    pub fn generate(&self) -> String {
        let mut data = format!(
            "accessKeyId={}&action={}&algorithm={}&nonce={}&timestamp={}",
            &self.id,
            &self.action,
            &self.algorithm.to_string(),
            self.nonce(),
            self.timestamp()
        );

        let signature = self.encrypt(&data);

        data.push_str("&signature=");
        data.push_str(&signature);

        data
    }
}
