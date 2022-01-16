#[derive(Debug, Clone)]
pub struct SecretsConfig {
    jwt: String,
    company: String,
    access_expire_in: usize,
}

impl SecretsConfig {
    pub fn new(jwt: String, company: String, access_expire_in: usize) -> Self {
        Self {
            jwt,
            company,
            access_expire_in,
        }
    }

    pub fn jwt(&self) -> &str {
        &self.jwt
    }

    pub fn company(&self) -> &str {
        &self.company
    }

    pub fn access_expire_in(&self) -> usize {
        self.access_expire_in
    }
}
