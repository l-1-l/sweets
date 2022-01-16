use common::{
    async_trait,
    chrono::{DateTime, Utc},
    models::{AggregateRoot, Avatar, Bio, Gender},
    pg::{map_rows, Pool, Row},
    Error, Result,
};
use std::{convert::TryFrom, sync::Arc};

use crate::{
    domain::user::{Birthdate, User, UserName, UserRepoExt, UserStatus},
    UserId,
};

impl TryFrom<Row> for User {
    type Error = Error;
    fn try_from(row: Row) -> Result<Self> {
        // let base = AggregateRoot::try_from(&row)?;
        let id: Vec<u8> = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let bio: Option<String> = row.try_get("bio")?;
        let birthdate: DateTime<Utc> = row.try_get("birthdate")?;
        let gender: String = row.try_get("gender")?;
        let avatar: Option<String> = row.try_get("avatar")?;
        let status: String = row.try_get("status")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let updated_at: Option<DateTime<Utc>> = row.try_get("updated_at")?;

        Ok(User::build(
            AggregateRoot::build(UserId::from(id), created_at, updated_at),
            UserName(name),
            bio.map(Bio),
            Birthdate(birthdate),
            Gender::build(&gender),
            avatar.map(Avatar),
            UserStatus::build(&status),
        ))
    }
}

pub struct UserRepo(pub Arc<Pool>);

#[async_trait]
impl UserRepoExt for UserRepo {
    async fn find_by_id(&self, id: &UserId) -> Result<User> {
        let client = self.0.get().await?;

        let row = client
            .query_one(
                "SELECT * FROM users
            WHERE id = $1",
                &[&id.as_ref()],
            )
            .await
            .map_err(|err| Error::not_found("user").wrap_raw(err))?;

        Ok(User::try_from(row)?)
    }

    async fn list_by_ids(&self, ids: &[UserId]) -> Result<Vec<User>> {
        let client = self.0.get().await?;
        let rows = client
            .query(
                "SELECT * FROM users
            WHERE id = ANY($1)",
                &[&ids.iter().map(|id| id.as_ref()).collect::<Vec<&[u8]>>()],
            )
            .await?;

        Ok(map_rows(rows, User::try_from)?)
    }

    async fn save(&self, user: &User) -> Result<()> {
        let client = self.0.get().await?;
        let base = user.base();
        let statement = r"
        INSERT INTO users
            (
                id,
                name,
                bio,
                birthdate,
                gender,
                avatar,
                status,
                created_at,
                updated_at
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8, $9)
            ON CONFLICT (id) DO UPDATE SET
                name = $2,
                bio = $3,
                birthdate = $4,
                gender = $5,
                avatar = $6,
                status = $7,
                updated_at = $9
        ";

        client
            .execute(
                statement,
                &[
                    &base.id().as_ref(),
                    &user.name().as_ref(),
                    &user.bio().map(|b| b.as_ref()),
                    &user.birthdate().as_ref(),
                    &user.gender().as_ref(),
                    &user.avatar().map(|x| x.as_ref()),
                    &user.status().as_ref(),
                    &base.created_at(),
                    &base.updated_at(),
                ],
            )
            .await
            .map_err(|e| {
                println!("user error: {}", e);
                e
            })?;

        Ok(())
    }
}
