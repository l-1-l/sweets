use std::sync::Arc;

use user_access::{
    container::UserAccessContainer,
    infra::repository::{
        account::AccountRepo, auth_code_inmem::AuthCodeInmemRepo,
        auth_token_inmem::AuthTokenInmemRepo, user::UserRepo,
    },
};

use common::{
    Config, SharedConfig, SharedLapinPool, SharedPgPool, SharedRedisPool,
};

#[derive(Clone)]
pub struct AppContainer {
    pub settings: Arc<Config>,
    pub user_access: UserAccessContainer,
}

impl AppContainer {
    pub async fn new(
        settings: SharedConfig,
        pg_pool: SharedPgPool,
        redis_pool: SharedRedisPool,
        _lapin_pool: SharedLapinPool,
    ) -> Self {
        // User Access
        let user_repo = Arc::new(UserRepo(pg_pool.clone()));
        let account_repo = Arc::new(AccountRepo(pg_pool));
        let auth_token_repo = Arc::new(AuthTokenInmemRepo(redis_pool.clone()));
        let auth_code_repo = Arc::new(AuthCodeInmemRepo(redis_pool));

        Self {
            settings,
            user_access: UserAccessContainer::new(
                account_repo,
                user_repo,
                auth_token_repo,
                auth_code_repo,
            ),
        }
    }
}
