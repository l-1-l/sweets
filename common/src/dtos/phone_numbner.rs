use crate::{models::PhoneNumber, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumberDto {
    pub prefix: String,
    pub mobile: String,
}

impl From<&PhoneNumber> for PhoneNumberDto {
    fn from(phone_number: &PhoneNumber) -> Self {
        PhoneNumberDto {
            prefix: phone_number.prefix().to_string(),
            mobile: phone_number.mobile().to_string(),
        }
    }
}

impl TryInto<PhoneNumber> for &PhoneNumberDto {
    type Error = Error;

    fn try_into(self) -> Result<PhoneNumber> {
        PhoneNumber::parse(&self.prefix, &self.mobile)
    }
}
