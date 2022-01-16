mod auth_code_repository;
mod authentication_service;
mod repository;
mod status;

pub use auth_code_repository::AuthCodeRepoExt;
pub use authentication_service::AuthenticationService;
pub use repository::AccountRepoExt;
pub use status::AccountStatus;

use crate::UserId;
use common::{
    models::{AggregateRoot, Email, Id, Password, PhoneNumber},
    Result,
};

pub type AccountId = Id;

#[derive(Debug, Clone)]
pub struct Account {
    base: AggregateRoot<AccountId>,
    user_id: Option<AccountId>,
    phone_number: PhoneNumber,
    email: Option<Email>,
    pwd: Option<Password>,
    status: AccountStatus,
}

impl Account {
    pub fn new(
        id: AccountId,
        phone: PhoneNumber,
        user_id: Option<AccountId>,
        email: Option<Email>,
        pwd: Option<Password>,
    ) -> Result<Self> {
        let account = Account {
            base: AggregateRoot::new(id),
            user_id,
            phone_number: phone,
            email,
            pwd,
            status: AccountStatus::Stagged,
        };

        Ok(account)
    }

    pub fn build(
        base: AggregateRoot<AccountId>,
        user_id: Option<AccountId>,
        phone: PhoneNumber,
        email: Option<Email>,
        pwd: Option<Password>,
        status: AccountStatus,
    ) -> Self {
        Self {
            base,
            user_id,
            phone_number: phone,
            email,
            pwd,
            status,
        }
    }

    pub fn base(&self) -> &AggregateRoot<AccountId> {
        &self.base
    }
    pub fn user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
    }

    pub fn set_user_id(&mut self, user_id: UserId) -> Result<()> {
        // TODO: check if user_id is already set, if so, send sms to user
        let is_active = self.user_id.is_some();

        if self.status == AccountStatus::Stagged && !is_active {
            self.status = AccountStatus::Active;
        }

        self.user_id = Some(user_id);

        self.base.update();

        Ok(())
    }

    pub fn phone_number(&self) -> &PhoneNumber {
        &self.phone_number
    }
    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }
    pub fn pwd(&self) -> Option<&Password> {
        self.pwd.as_ref()
    }
    pub fn status(&self) -> &AccountStatus {
        &self.status
    }
    pub fn set_status(&mut self, status: AccountStatus) {
        self.status = status;
        self.base.update();
    }
}
