use common::{
    dtos::PhoneNumberDto, models::PhoneNumber, random, Config, Error, Result,
};
use serde::{Deserialize, Serialize};
use uni_sms::{
    message::{MessageKind, UniMessageSms, UniMobile},
    signature::Signature,
    Algorithm,
};

use crate::{
    domain::account::{AccountRepoExt, AuthCodeRepoExt},
    types::AuthAction,
};

#[derive(Serialize)]
struct SMSParams<'a> {
    code: &'a str,
    ttl: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CodeCommand {
    email: Option<String>,
    phone_number: Option<PhoneNumberDto>,
}

#[derive(Serialize)]
pub struct CodeResponse {
    pub is_new: bool,
}

pub struct Code<'a> {
    account_repo: &'a dyn AccountRepoExt,
    auth_code_repo: &'a dyn AuthCodeRepoExt,
}

impl<'a> Code<'a> {
    pub fn new(
        account_repo: &'a dyn AccountRepoExt,
        auth_code_repo: &'a dyn AuthCodeRepoExt,
    ) -> Self {
        Self {
            account_repo,
            auth_code_repo,
        }
    }

    pub async fn exec(&self, cmd: &CodeCommand) -> Result<CodeResponse> {
        if cmd.email.is_none() && cmd.phone_number.is_none() {
            return Err(Error::bad_format("code")
                .set_message("email or phone_number must be specified"));
        }

        if let Some(phone) = &cmd.phone_number {
            return self.send_code_by_phone(phone.try_into()?).await;
        }

        // TODO: email

        Err(Error::bad_format("code").set_message("not implemented"))
    }

    async fn send_code_by_phone(
        &self,
        phone: PhoneNumber,
    ) -> Result<CodeResponse> {
        let account = self.account_repo.find_by_phone(&phone).await?;

        let action = match account {
            Some(_) => AuthAction::Signin,
            None => AuthAction::Signup,
        };

        let is_freed = self
            .auth_code_repo
            .is_freed(&action, &phone.normilize())
            .await?;

        if is_freed {
            return Err(Error::forbidden("auth.code.sms").set_message(
                format!(
                    "SMS Code already sended to phone {}",
                    &phone.normilize()
                ),
            ));
        }

        let count = self
            .auth_code_repo
            .counter(&action, &phone.normilize())
            .await?;

        if count > 10 {
            return Err(Error::unauthorized().set_message(format!(
                "Too many attempts for phone {}",
                phone.mobile()
            )));
        }

        let code = random::code();
        let sms_conf = Config::get().uni_sms();

        let signature = Signature {
            id: sms_conf.key.clone(),
            secret: sms_conf.secret.clone(),
            algorithm: Algorithm::HmacSha256,
            action: "sms.message.send".to_string(),
        };

        let params = SMSParams {
            code: &code,
            ttl: 2,
        };

        UniMessageSms::new(
            signature,
            sms_conf.sign.clone(),
            MessageKind::Template("pub_verif_ttl3".to_string()),
        )
        .send_with_params(
            UniMobile::Single(format!("{}{}", phone.prefix(), phone.mobile(),)),
            &params,
        )
        .await?;

        self.auth_code_repo
            .save(&action, &phone.normilize(), &code)
            .await?;

        Ok(CodeResponse {
            is_new: action == AuthAction::Signup,
        })
    }
}
