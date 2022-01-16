mod aggregate_root;
mod avatar;
mod bio;
mod blurhash;
mod email;
mod gender;
mod id;
mod image;
mod password;
mod phone;

pub use aggregate_root::AggregateRoot;
pub use avatar::Avatar;
pub use bio::Bio;
pub use blurhash::Blurhash;
pub use email::Email;
pub use gender::Gender;
pub use id::Id;
pub use image::{Image, ImageList};
pub use password::Password;
pub use phone::PhoneNumber;

pub enum EmailOrPhoneNumber {
    Email(Email),
    Mobile(PhoneNumber),
}
