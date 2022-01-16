use crate::domain::account::AccountId;
use common::{
    async_trait,
    models::{Email, PhoneNumber},
    Result,
};

use super::Account;

#[async_trait]
pub trait AccountRepoExt: Sync + Send {
    async fn next_id(&self) -> Result<AccountId> {
        Ok(AccountId::next())
    }
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>>;
    async fn find_by_phone(&self, phone: &PhoneNumber) -> Result<Option<Account>>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<Account>>;
    async fn save(&self, data: &Account) -> Result<()>;
    async fn hard_delete(&self, id: &AccountId) -> Result<()>;
}
