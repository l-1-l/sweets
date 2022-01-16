use crate::Result;

#[derive(Debug, Clone)]
pub struct PhoneNumber {
    prefix: String,
    mobile: String,
}

impl PhoneNumber {
    pub fn build(prefix: String, mobile: String) -> Self {
        Self { prefix, mobile }
    }
    pub fn parse<C, M>(prefix: C, mobile: M) -> Result<Self>
    where
        C: std::fmt::Display,
        M: std::fmt::Display,
    {
        // TODO: validate phone number
        let prefix = prefix.to_string();
        let mobile = mobile.to_string();

        Ok(PhoneNumber { prefix, mobile })
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn mobile(&self) -> &str {
        &self.mobile
    }

    pub fn normilize(&self) -> String {
        format!("+{}{}", &self.prefix, &self.mobile)
    }
}
