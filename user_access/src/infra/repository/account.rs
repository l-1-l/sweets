use crate::{
    domain::account::{Account, AccountId, AccountRepoExt, AccountStatus},
    UserId,
};
use common::{
    async_trait,
    chrono::{DateTime, Utc},
    models::{AggregateRoot, Email, Password, PhoneNumber},
    pg::{Pool, Row},
    Error, Result,
};
use std::sync::Arc;

impl TryFrom<&Row> for Account {
    type Error = Error;

    fn try_from(r: &Row) -> Result<Self> {
        let id: Vec<u8> = r.try_get("id")?;
        let user_id: Option<Vec<u8>> = r.try_get("user_id")?;
        let mobile: String = r.try_get("mobile")?;
        let prefix: String = r.try_get("mobile_prefix")?;
        let email: Option<String> = r.try_get("email")?;
        let pwd: Option<String> = r.try_get("hash")?;
        let staus: i16 = r.try_get("status")?;
        let created_at: DateTime<Utc> = r.try_get("created_at")?;
        let updated_at: Option<DateTime<Utc>> = r.try_get("updated_at")?;

        Ok(Account::build(
            AggregateRoot::build(AccountId::from(id), created_at, updated_at),
            user_id.map(UserId::from),
            PhoneNumber::build(prefix, mobile),
            email.map(Email),
            pwd.map(Password),
            AccountStatus::from(staus),
        ))
    }
}

pub struct AccountRepo(pub Arc<Pool>);

#[async_trait]
impl AccountRepoExt for AccountRepo {
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>> {
        let client = self.0.get().await?;
        client
            .query_opt("SELECT * FROM accounts WHERE id = $1", &[&id.as_ref()])
            .await?
            .map(|r| Account::try_from(&r))
            .transpose()
    }

    async fn find_by_phone(
        &self,
        phone: &PhoneNumber,
    ) -> Result<Option<Account>> {
        let client = self.0.get().await?;
        client
            .query_opt(
                r"SELECT * FROM accounts WHERE mobile = $1 AND mobile_prefix = $2",
                &[&phone.mobile(), &phone.prefix()],
            )
            .await?
            .map(|r| Account::try_from(&r))
            .transpose()
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<Account>> {
        let client = self.0.get().await?;
        client
            .query_opt(
                "SELECT * FROM accounts WHERE email = $1",
                &[&email.as_ref()],
            )
            .await?
            .map(|r| Account::try_from(&r))
            .transpose()
    }

    async fn save(&self, account: &Account) -> Result<()> {
        let client = self.0.get().await?;
        let base = account.base();
        let statement = "
      INSERT INTO accounts
        (
          id,
          user_id,
          mobile,
          mobile_prefix,
          email,
          hash,
          status,
          created_at,
          updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (id) DO UPDATE
        SET
          user_id = $2,
          mobile = $3,
          mobile_prefix = $4,
          email = $5,
          hash = $6,
          status = $7,
          updated_at = $9
      ";

        client
            .execute(
                statement,
                &[
                    &base.id().as_ref(),
                    &account.user_id().map(|x| x.as_ref()),
                    &account.phone_number().mobile(),
                    &account.phone_number().prefix(),
                    &account.email().map(|x| x.as_ref()),
                    &account.pwd().map(|x| x.as_ref()),
                    &account.status().as_i16(),
                    &base.created_at(),
                    &base.updated_at(),
                ],
            )
            .await?;

        Ok(())
    }

    async fn hard_delete(&self, id: &AccountId) -> Result<()> {
        let client = self.0.get().await?;
        client
            .execute("DELETE FROM accounts WHERE id = $1", &[&id.as_ref()])
            .await?;

        Ok(())
    }
}
