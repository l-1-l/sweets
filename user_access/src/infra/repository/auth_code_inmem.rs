use crate::{domain::account::AuthCodeRepoExt, types::AuthAction};
use common::{
    async_trait,
    chrono::Utc,
    deadpool_redis::{redis::AsyncCommands, Pool},
    Result,
};
use std::sync::Arc;
pub struct AuthCodeInmemRepo(pub Arc<Pool>);

impl AuthCodeInmemRepo {
    fn key(&self, action: &str, name: &str) -> String {
        format!("{}-{}", action, name)
    }
}

#[async_trait]
impl AuthCodeRepoExt for AuthCodeInmemRepo {
    async fn save(
        &self,
        ac: &AuthAction,
        phone: &str,
        code: &str,
    ) -> Result<()> {
        let key = &self.key(&ac.to_string(), phone);
        let counter_key = self.key(key, "counter");
        let freeze_key = self.key(key, "freeze");

        let mut conn = self.0.get().await?;

        conn.set_ex(key, &code, 60 * 5).await?;

        // 以天为单位统计
        let is_counter_exists: bool = conn.exists(&counter_key).await?;
        if is_counter_exists {
            conn.incr(&counter_key, 1).await?;
        } else {
            conn.set_ex(counter_key, 1, 60 * 60 * 24).await?;
        }

        // 分钟锁
        let is_freeze_exists: bool = conn.exists(&freeze_key).await?;
        let now = Utc::now().timestamp();
        if !is_freeze_exists {
            conn.set_ex(freeze_key, now, 120).await?;
        }

        Ok(())
    }

    async fn get(
        &self,
        ac: &AuthAction,
        phone: &str,
    ) -> Result<Option<String>> {
        let key = self.key(&ac.to_string(), phone);
        let mut conn = self.0.get().await?;

        conn.get(key).await.map_err(|e| e.into())
    }

    async fn is_freed(&self, ac: &AuthAction, phone: &str) -> Result<bool> {
        let key = &self.key(&self.key(&ac.to_string(), phone), "freeze");
        let mut conn = self.0.get().await?;

        let r: bool = conn.exists(key).await?;

        Ok(r)
    }

    async fn counter(&self, ac: &AuthAction, phone: &str) -> Result<i16> {
        let key = &self.key(&ac.to_string(), phone);
        let counter_key = self.key(key, "counter");

        let mut conn = self.0.get().await?;
        let res: Option<i16> = conn.get(counter_key).await?;

        match res {
            Some(v) => Ok(v),
            None => Ok(-1),
        }
    }
}
