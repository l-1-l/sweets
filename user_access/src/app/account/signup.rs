mod active;

pub use active::*;

use crate::{
    app::dtos::AccountDto, domain::account::AuthenticationService,
    types::AuthAction,
};
use common::{dtos::PhoneNumberDto, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Deserialize)]
pub struct SignupCommand {
    pub code: String,
    pub phone_number: PhoneNumberDto,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub account: AccountDto,
    pub token: String,
}

pub struct Signup<'a> {
    authentication_srv: &'a AuthenticationService,
}

impl<'a> Signup<'a> {
    pub fn new(authentication_srv: &'a AuthenticationService) -> Self {
        Self { authentication_srv }
    }

    pub async fn exec(&self, cmd: SignupCommand) -> Result<SignupResponse> {
        let phone = &cmd.phone_number;

        match self
            .authentication_srv
            .verify_sms(&AuthAction::Signup, &phone.try_into()?, &cmd.code)
            .await
        {
            Ok((account, token)) => Ok(SignupResponse {
                account: AccountDto::from(&account),
                token: token.to_string(),
            }),

            Err(e) => Err(e),
        }
    }
}
