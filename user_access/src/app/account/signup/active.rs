use crate::{
    app::dtos::{AccountDto, UserDto},
    domain::{
        account::{AccountRepoExt, AuthenticationService},
        token::Token,
        user::{Birthdate, User, UserName, UserRepoExt},
    },
};

use common::{
    chrono::{DateTime, Utc},
    models::Gender,
    Error, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MobileNumber {
    pub number: String,
    pub country_code: u8,
}

#[derive(Deserialize)]
pub struct ActiveCommand {
    pub username: String,
    pub birthdate: DateTime<Utc>,
    pub gender: String,
}

#[derive(Serialize)]
pub struct ActiveResponse {
    user: UserDto,
    account: AccountDto,
}

pub struct Active<'a> {
    access_token: &'a str,
    account_repo: &'a dyn AccountRepoExt,
    user_repo: &'a dyn UserRepoExt,
    authentication_srv: &'a AuthenticationService,
}

impl<'a> Active<'a> {
    pub fn new(
        access_token: &'a str,
        account_repo: &'a dyn AccountRepoExt,
        user_repo: &'a dyn UserRepoExt,
        authentication_srv: &'a AuthenticationService,
    ) -> Self {
        Self {
            access_token,
            account_repo,
            user_repo,
            authentication_srv,
        }
    }

    pub async fn exec(&self, cmd: ActiveCommand) -> Result<ActiveResponse> {
        let token = Token::new(&self.access_token);

        let account_id =
            self.authentication_srv.verify_access_token(&token).await?;
        let mut account = self
            .account_repo
            .find_by_id(&account_id)
            .await?
            .ok_or_else(|| Error::forbidden("active"))?;

        if account.user_id().is_some() {
            return Err(Error::forbidden("active").set_message("user is exist"));
        }

        let user_id = self.user_repo.next_id().await?;
        let user = User::new(
            user_id,
            UserName::parse(cmd.username)?,
            None,
            Birthdate::parse(cmd.birthdate)?,
            Gender::parse(&cmd.gender)?,
            None,
        )?;

        account.set_user_id(user_id)?;

        self.user_repo.save(&user).await?;

        self.account_repo.save(&account).await?;

        Ok(ActiveResponse {
            user: UserDto::from(&user),
            account: AccountDto::from(&account),
        })
    }
}
