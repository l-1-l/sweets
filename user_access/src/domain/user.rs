mod birthdate;
// mod custom_status;
mod name;
mod repository;
mod status;

pub use birthdate::Birthdate;
// pub use custom_status::CustomStatus;
pub use name::UserName;
pub use repository::UserRepoExt;
pub use status::UserStatus;

pub type UserId = Id;

use common::{
    models::{AggregateRoot, Avatar, Bio, Gender, Id},
    Result,
};

#[derive(Debug, Clone)]
pub struct User {
    base: AggregateRoot<UserId>,
    name: UserName,
    bio: Option<Bio>,
    birthdate: Birthdate,
    gender: Gender,
    avatar: Option<Avatar>,
    status: UserStatus,
    // custom_status: Option<CustomStatus>,
}

impl User {
    pub fn new(
        id: UserId,
        name: UserName,
        bio: Option<Bio>,
        birthdate: Birthdate,
        gender: Gender,
        avatar: Option<Avatar>,
    ) -> Result<User> {
        let user = User {
            base: AggregateRoot::new(id),
            name,
            bio,
            birthdate,
            gender,
            avatar,
            status: UserStatus::Online,
        };

        Ok(user)
    }

    pub fn build(
        base: AggregateRoot<UserId>,
        name: UserName,
        bio: Option<Bio>,
        birthdate: Birthdate,
        gender: Gender,
        avatar: Option<Avatar>,
        status: UserStatus,
    ) -> Self {
        Self {
            base,
            name,
            bio,
            birthdate,
            gender,
            avatar,
            status,
        }
    }

    pub fn base(&self) -> &AggregateRoot<UserId> {
        &self.base
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn set_name(&mut self, name: UserName) {
        self.name = name;
    }

    pub fn bio(&self) -> Option<&Bio> {
        self.bio.as_ref()
    }

    pub fn set_bio(&mut self, bio: Bio) {
        self.bio = Some(bio);
    }

    pub fn birthdate(&self) -> &Birthdate {
        &self.birthdate
    }

    pub fn set_birthdate(&mut self, birthdate: Birthdate) {
        self.birthdate = birthdate;
    }

    pub fn gender(&self) -> &Gender {
        &self.gender
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn avatar(&self) -> Option<&Avatar> {
        self.avatar.as_ref()
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }
}
