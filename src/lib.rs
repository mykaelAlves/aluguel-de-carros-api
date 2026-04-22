pub mod entities;
pub mod middleware;
pub mod routes;

pub const EXPECTED_USER_TOKEN: &str = "tokenFERA@";
pub const EXPECTED_ADMIN_TOKEN: &str = "adminTokenFERA@";

#[derive(Clone, Default)]
pub struct AppState {
    pub usuarios: Vec<entities::Usuario>,
    pub produtos: Vec<entities::Produto>,
}
