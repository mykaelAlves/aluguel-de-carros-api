use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use axum::http::StatusCode;
use sha2::Digest;
use tokio::sync::RwLock;

use crate::entities::{Produto, ProdutoFiltro, Role, UsuarioLogin, UsuarioRegistro};
use crate::{AppState, entities::Usuario};

type Token = &'static str;
type RwAppState = Arc<RwLock<AppState>>;

pub async fn root() -> StatusCode {
    StatusCode::OK
}

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

#[axum::debug_handler]
pub async fn login(
    State(state): State<RwAppState>, 
    Json(usuario_login): Json<UsuarioLogin>
) -> Result<(StatusCode, Token), StatusCode> 
{    
    let rcv_hashed_pwd = hex::encode(sha2::Sha256::digest(usuario_login.password.as_bytes()));

    state.read().await.usuarios
        .iter()
        .find(|u| u.email == usuario_login.email && u.pwd_hash == rcv_hashed_pwd)
        .map(|_| (StatusCode::OK, "tokenFERA@"))
        .ok_or(StatusCode::UNAUTHORIZED)
}

#[axum::debug_handler]
pub async fn register(
    State(state): State<RwAppState>, Json(usuario_registro): Json<UsuarioRegistro>
) -> Result<(StatusCode, Token), StatusCode> 
{
    if state.read().await.usuarios
        .iter()
        .find(|u| u.email == usuario_registro.email)
        .is_some() {
            return Err(StatusCode::CONFLICT);
    }

    state.write().await.usuarios.push(Usuario {
        id: state.read().await.usuarios.len() as u32 + 1,
        nome: usuario_registro.nome,
        telefone: usuario_registro.telefone,
        email: usuario_registro.email,
        pwd_hash: hex::encode(sha2::Sha256::digest(usuario_registro.password.as_bytes())),
        role: Role::Cliente,
    });

    Ok((StatusCode::CREATED, "tokenFERA@"))
}

#[axum::debug_handler]
pub async fn produtos_list(
    State(state): State<RwAppState>, 
    Query(filtros): Query<ProdutoFiltro>
) -> Result<Json<Vec<Produto>>, StatusCode> 
{
    let guard = state.read().await;

    let produtos_filtrados = guard.produtos
        .iter()
        .filter(|p| {
            (filtros.marca.as_ref().map_or(true, |m| &p.marca == m)) &&
            (filtros.modelo.as_ref().map_or(true, |m| &p.modelo == m)) &&
            (filtros.ano.map_or(true, |a| p.ano == a)) &&
            (filtros.cor.as_ref().map_or(true, |c| &p.cor == c)) &&
            (filtros.disponivel.map_or(true, |d| p.disponivel == d))
        })
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(produtos_filtrados)) 
}

#[axum::debug_handler]
pub async fn produto_create(
    State(state): State<RwAppState>, 
    Json(novo_produto): Json<Produto>
) -> Result<StatusCode, StatusCode> 
{
    let mut guard = state.write().await;

    if guard.produtos.iter().any(|p| p.sku == novo_produto.sku) {
        return Err(StatusCode::CONFLICT);
    }

    guard.produtos.push(novo_produto);
    Ok(StatusCode::CREATED)
}

#[axum::debug_handler]
pub async fn produto_sku(
    State(state): State<RwAppState>, 
    axum::extract::Path(sku): axum::extract::Path<String>
) -> Result<Json<Produto>, StatusCode> 
{
    let guard = state.read().await;

    guard.produtos
        .iter()
        .find(|p| p.sku == sku)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn delete_account(
    State(state): State<RwAppState>,
    Query(nome): Query<String>
) -> impl IntoResponse {
    let mut guard = state.write().await;

    if let Some(pos) = guard.usuarios.iter().position(|u| u.nome == nome) {
        guard.usuarios.remove(pos);
        return StatusCode::OK
    }

    StatusCode::NOT_FOUND
}

pub async fn alugar_produto(
    State(state): State<RwAppState>,
    Path(sku): Path<String>
) -> impl IntoResponse {
    _produto_alugar_devolver(state, sku, true).await
}

pub async fn devolver_produto(
    State(state): State<RwAppState>,
    Path(sku): Path<String>
) -> impl IntoResponse 
{
    _produto_alugar_devolver(state, sku, false).await
}

async fn _produto_alugar_devolver(
    state: RwAppState,
    sku: String,
    alugar: bool
) -> impl IntoResponse {
    let mut guard = state.write().await;

    if let Some(produto) = guard.produtos.iter_mut().find(|p| p.sku == sku) {
        if produto.disponivel == alugar {
            produto.disponivel = !alugar;
            return StatusCode::OK
        } else {
            return StatusCode::CONFLICT
        }
    }

    StatusCode::NOT_FOUND
}

pub async fn produto_delete(
    State(state): State<RwAppState>, 
    Path(sku): Path<String>
) -> impl IntoResponse {
    let mut guard = state.write().await;

    if let Some(pos) = guard.produtos.iter().position(|p| p.sku == sku) {
        guard.produtos.remove(pos);
        return StatusCode::OK
    }

    StatusCode::NOT_FOUND
}
