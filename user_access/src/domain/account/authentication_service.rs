use std::sync::Arc;

use crate::{
    domain::{
        account::{Account, AccountRepoExt, AuthCodeRepoExt},
        token::{Token, TokenService},
    },
    types::AuthAction,
};
use common::{
    models::{EmailOrPhoneNumber, PhoneNumber},
    Error, Result,
};

use super::AccountId;

pub struct AuthenticationService {
    account_repo: Arc<dyn AccountRepoExt>,
    auth_code_repo: Arc<dyn AuthCodeRepoExt>,
    token_srv: Arc<TokenService>,
}

impl AuthenticationService {
    pub fn new(
        account_repo: Arc<dyn AccountRepoExt>,
        auth_code_repo: Arc<dyn AuthCodeRepoExt>,
        token_srv: Arc<TokenService>,
    ) -> Self {
        AuthenticationService {
            account_repo,
            auth_code_repo,
            token_srv,
        }
    }

    pub async fn verify_access_token(
        &self,
        token: &Token,
    ) -> Result<AccountId> {
        self.token_srv.validate(token).await
    }

    pub async fn verify_pwd(
        &self,
        email_or_phone: &EmailOrPhoneNumber,
        pwd: &str,
    ) -> Result<(Account, Token)> {
        let account = match email_or_phone {
            EmailOrPhoneNumber::Email(email) => {
                self.account_repo.find_by_email(email).await
            }
            EmailOrPhoneNumber::Mobile(phone) => {
                self.account_repo.find_by_phone(phone).await
            }
        }?
        .ok_or_else(Error::unauthorized)?; // 没有设置密码

        match account.pwd() {
            Some(password) => {
                if password.verify(pwd.as_bytes())? {
                    let token =
                        self.token_srv.create(account.base().id()).await?;

                    Ok((account, token))
                } else {
                    Err(Error::unauthorized())
                }
            }
            None => Err(Error::unauthorized()),
        }
    }

    pub async fn verify_sms(
        &self,
        action: &AuthAction,
        phone: &PhoneNumber,
        sms_code: &str,
    ) -> Result<(Account, Token)> {
        let auth_code = self
            .auth_code_repo
            .get(action, &phone.normilize())
            .await?
            .ok_or_else(|| {
                Error::unauthorized() // 验证码已失效
                    .set_message("Please Resend Code")
            })?;

        if auth_code != sms_code {
            return Err(Error::unauthorized()
                .set_message("Please Provide The Right Code"));
        }

        let account = match self.account_repo.find_by_phone(phone).await? {
            Some(account) => {
                match action {
                    AuthAction::Signup => {
                        // TODO: 需要设计一套流程, 鉴别用户是否为新用户,
                        //  不成熟的想法
                        // 1. 邮箱+密码验证
                        // 2. 看账户x(结合country_code与各国手机号回收的日期规则)天内是否活跃
                        // 3. 保留已存在账户并解绑手机号
                        return Err(Error::forbidden("credentials")
                            .set_code("")
                            .set_message("Phone Number Has Been Registered"));
                    }
                    AuthAction::Signin => account,
                }
            }
            None => {
                let id = self.account_repo.next_id().await?;
                let account =
                    Account::new(id, phone.clone(), None, None, None)?;

                self.account_repo.save(&account).await?;

                account
            }
        };

        let token = self.token_srv.create(account.base().id()).await?;

        Ok((account, token))
    }
}
