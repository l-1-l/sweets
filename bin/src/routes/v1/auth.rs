use axum::routing::post;
use axum::{extract::Extension, Json, Router};
use user_access::app::account::code::{Code, CodeCommand, CodeResponse};
use user_access::app::account::signin::{
    Signin, SigninCommand, SigninResponse,
};
use user_access::app::account::signup::{
    Active, ActiveCommand, ActiveResponse, Signup, SignupCommand,
    SignupResponse,
};

use crate::container::AppContainer;
use crate::extractors::BearerToken;
use crate::prelude::*;

async fn send_auth_code(
    Json(cmd): Json<CodeCommand>,
    Extension(ctx): Extension<AppContainer>,
) -> ApiResult<CodeResponse> {
    let data = Code::new(
        ctx.user_access.account_repo(),
        ctx.user_access.auth_code_repo(),
    )
    .exec(&cmd)
    .await?;

    created(data)
}

async fn signin(
    Json(cmd): Json<SigninCommand>,
    Extension(ctx): Extension<AppContainer>,
) -> ApiResult<SigninResponse> {
    let data = Signin::new(
        ctx.user_access.user_repo(),
        ctx.user_access.authentication_srv(),
    )
    .exec(cmd)
    .await?;

    ok(data)
}

async fn signup(
    Json(cmd): Json<SignupCommand>,
    Extension(ctx): Extension<AppContainer>,
) -> ApiResult<SignupResponse> {
    let data = Signup::new(ctx.user_access.authentication_srv())
        .exec(cmd)
        .await?;

    created(data)
}

async fn active(
    token: BearerToken,
    Json(cmd): Json<ActiveCommand>,
    Extension(ctx): Extension<AppContainer>,
) -> ApiResult<ActiveResponse> {
    let data = Active::new(
        token.as_ref(),
        ctx.user_access.account_repo(),
        ctx.user_access.user_repo(),
        ctx.user_access.authentication_srv(),
    )
    .exec(cmd)
    .await?;

    ok(data)
}

pub fn init() -> Router {
    Router::new()
        .route("/code", post(send_auth_code))
        .route("/signup", post(signup))
        .route("/active", post(active))
}
