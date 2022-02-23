use common::{
    chrono::{DateTime, Utc},
    dtos::PhoneNumberDto,
};
use serde::Serialize;

use crate::domain::{account::Account, user::User};

// #[derive(Serialize)]
// pub struct UserSettingsDto {
//     language: String,
// post_viewing_condition: i16,
// }

#[derive(Serialize)]
pub struct UserDto {
    id: String,
    name: String,
    bio: Option<String>,
    birthdate: DateTime<Utc>,
    gender: String,
    avatar: Option<String>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        Self {
            id: user.base().id().to_string(),
            name: user.name().to_string(),
            bio: user.bio().map(|bio| bio.to_string()),
            birthdate: *user.birthdate().as_ref(),
            gender: user.gender().to_string(),
            avatar: user.avatar().map(|avatar| avatar.to_string()),
            status: user.status().to_string(),
            created_at: *user.base().created_at(),
            updated_at: user.base().updated_at().copied(),
        }
    }
}

#[derive(Serialize)]
pub struct AccountDto {
    id: String,
    user_id: Option<String>,
    phone_number: PhoneNumberDto,
    email: Option<String>,
    status: i16,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<&Account> for AccountDto {
    fn from(account: &Account) -> Self {
        Self {
            id: account.base().id().to_string(),
            user_id: account.user_id().map(|user_id| user_id.to_string()),
            phone_number: PhoneNumberDto::from(account.phone_number()),
            email: account.email().map(|email| email.to_string()),
            status: account.status().as_i16(),
            created_at: *account.base().created_at(),
            updated_at: account.base().updated_at().copied(),
        }
    }
}
