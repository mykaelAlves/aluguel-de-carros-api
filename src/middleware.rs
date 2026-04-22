use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use tracing::debug;

use crate::{EXPECTED_ADMIN_TOKEN, EXPECTED_USER_TOKEN};

#[axum::debug_middleware]
pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let rcv_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_str| auth_str.strip_prefix("Bearer "));

    debug!("Token recebido: {:?}.", rcv_token);

    if rcv_token != Some(EXPECTED_USER_TOKEN)
        && rcv_token != Some(EXPECTED_ADMIN_TOKEN)
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

#[axum::debug_middleware]
pub async fn auth_adm_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let rcv_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_str| auth_str.strip_prefix("Bearer "));

    debug!("Token recebido: {:?}.", rcv_token);

    if rcv_token != Some(EXPECTED_ADMIN_TOKEN) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
