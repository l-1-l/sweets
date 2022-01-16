use common::{Error, Result};

#[derive(Debug, Clone)]
pub struct AccountName(pub String);

impl AccountName {
    pub fn parse<S: Into<String>>(val: S) -> Result<AccountName> {
        let name = val.into();
        let len = name.len();

        if !(4..16).contains(&len) {
            return Err(Error::bad_format("name")
                .set_message("account name must be between 4 and 16 characters"));
        }

        Ok(AccountName(name))
    }
}

impl AsRef<str> for AccountName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
