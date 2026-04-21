use axum::http::{StatusCode, header};
use axum::middleware::Next;
use axum::response::Response;
use axum::extract::Request;
use crate::EXPECTED_USER_TOKEN;
use crate::EXPECTED_ADMIN_TOKEN;

#[axum::debug_middleware]
pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    _auth_middleware(req, next, EXPECTED_USER_TOKEN).await
}

#[axum::debug_middleware]
pub async fn auth_adm_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    _auth_middleware(req, next, EXPECTED_ADMIN_TOKEN).await
}

async fn _auth_middleware(
    req: Request, next: Next, expected_token: &str
) -> Result<Response, StatusCode> 
{
    let rcv_token = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_str| auth_str.strip_prefix("Bearer "));

    if rcv_token != Some(expected_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}