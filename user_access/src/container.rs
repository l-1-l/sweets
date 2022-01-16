use std::sync::Arc;

use crate::{
    domain::{
        account::{AccountRepoExt, AuthCodeRepoExt, AuthenticationService},
        token::{TokenRepositoryExt, TokenService},
        user::UserRepoExt,
    },
    infra::service::JWTEncoder,
};

#[derive(Clone)]
pub struct UserAccessContainer {
    account_repo: Arc<dyn AccountRepoExt>,
    user_repo: Arc<dyn UserRepoExt>,
    auth_token_repo: Arc<dyn TokenRepositoryExt>,
    auth_code_repo: Arc<dyn AuthCodeRepoExt>,
    authentication_srv: Arc<AuthenticationService>,
}

impl UserAccessContainer {
    pub fn new(
        account_repo: Arc<dyn AccountRepoExt>,
        user_repo: Arc<dyn UserRepoExt>,
        token_repo: Arc<dyn TokenRepositoryExt>,
        auth_code_repo: Arc<dyn AuthCodeRepoExt>,
    ) -> Self {
        let token_encoder = Arc::new(JWTEncoder::default());
        let token_srv =
            Arc::new(TokenService::new(token_repo.clone(), token_encoder));

        let authentication_srv = Arc::new(AuthenticationService::new(
            account_repo.clone(),
            auth_code_repo.clone(),
            token_srv,
        ));

        Self {
            account_repo,
            user_repo,
            auth_token_repo: token_repo,
            auth_code_repo,
            authentication_srv,
        }
    }

    pub fn account_repo(&self) -> &dyn AccountRepoExt {
        self.account_repo.as_ref()
    }

    pub fn user_repo(&self) -> &dyn UserRepoExt {
        self.user_repo.as_ref()
    }

    pub fn token_repo(&self) -> &dyn TokenRepositoryExt {
        self.auth_token_repo.as_ref()
    }

    pub fn auth_code_repo(&self) -> &dyn AuthCodeRepoExt {
        self.auth_code_repo.as_ref()
    }

    pub fn authentication_srv(&self) -> &AuthenticationService {
        self.authentication_srv.as_ref()
    }
}
