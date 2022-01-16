use crate::{
    app::dtos::{AccountDto, UserDto},
    domain::{
        account::{Account, AuthenticationService},
        token::Token,
        user::UserRepoExt,
    },
    types::AuthAction,
};
use common::{
    dtos::PhoneNumberDto,
    models::{Email, EmailOrPhoneNumber, PhoneNumber},
    Error, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SigninCommand {
    pub email: Option<String>,
    pub phone_number: Option<PhoneNumberDto>,
    pub code: Option<String>,
    pub pwd: Option<String>,
}

/// 如果用户不存在， 则需要激活 补全用户资料
#[derive(Serialize)]
pub struct SigninResponse {
    user: Option<UserDto>,
    account: AccountDto,
    access_token: String,
}

pub struct Signin<'a> {
    user_repo: &'a dyn UserRepoExt,
    authentication_srv: &'a AuthenticationService,
}

impl<'a> Signin<'a> {
    pub fn new(
        user_repo: &'a dyn UserRepoExt,
        authentication_srv: &'a AuthenticationService,
    ) -> Self {
        Signin {
            user_repo,
            authentication_srv,
        }
    }

    pub async fn exec(&self, cmd: SigninCommand) -> Result<SigninResponse> {
        if let Some(code) = cmd.code {
            return match &cmd.phone_number {
                Some(phone) => self.with_sms(phone.try_into()?, code).await,
                None => Err(Error::bad_format("signin")
                    .set_message("phone number is required")
                    .set_status(200)),
            };
        }

        if let Some(pwd) = cmd.pwd {
            if let Some(email) = cmd.email {
                let email = EmailOrPhoneNumber::Email(Email::parse(email)?);
                return self.with_pwd(email, pwd).await;
            }

            if let Some(phone) = cmd.phone_number {
                let phone_number = EmailOrPhoneNumber::Mobile(
                    PhoneNumber::parse(phone.prefix, phone.mobile)?,
                );
                return self.with_pwd(phone_number, pwd).await;
            }

            return Err(Error::bad_format("signin")
                .set_message("email or phone number is required"));
        }

        Err(Error::bad_format("signin")
            .set_message("password or phone number is required"))
    }

    async fn with_pwd(
        &self,
        email_or_phone: EmailOrPhoneNumber,
        pwd: String,
    ) -> Result<SigninResponse> {
        let (account, access_token) = self
            .authentication_srv
            .verify_pwd(&email_or_phone, &pwd)
            .await?;

        self.ok((account, access_token)).await
    }

    async fn with_sms(
        &self,
        phone: PhoneNumber,
        code: String,
    ) -> Result<SigninResponse> {
        let (account, access_token) = self
            .authentication_srv
            .verify_sms(&AuthAction::Signin, &phone, &code)
            .await?;

        self.ok((account, access_token)).await
    }

    async fn ok(
        &self,
        (account, access_token): (Account, Token),
    ) -> Result<SigninResponse> {
        let mut user: Option<UserDto> = None;

        if let Some(uid) = account.user_id() {
            let u = self.user_repo.find_by_id(uid).await?;
            user = Some(UserDto::from(&u));
        }

        Ok(SigninResponse {
            user,
            account: AccountDto::from(&account),
            access_token: access_token.to_string(),
        })
    }
}
