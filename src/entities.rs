use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Cliente,
    Administrador,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Usuario {
    pub id: u32,
    pub nome: String,
    pub telefone: String,
    pub email: String,
    pub pwd_hash: String,
    pub role: Role,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UsuarioRegistro {
    pub nome: String,
    pub telefone: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UsuarioLogin {
    pub email: String,
    pub password: String,
}

type SKU = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Produto {
    pub sku: SKU,
    pub marca: String,
    pub modelo: String,
    pub cor: String,
    pub ano: u16,
    pub disponivel: bool,
}

impl Produto {
    pub fn new(marca: &str, modelo: &str, ano: u16, cor: &str) -> Self {
        let filtered_marca = marca.replace(" ", "");
        let filtered_modelo = modelo.replace(" ", "");

        let sku = format!("{}_{}_{}_{}", filtered_marca, filtered_modelo, ano, cor);
        Produto {
            sku,
            marca: marca.to_string(),
            modelo: modelo.to_string(),
            cor: cor.to_string(),
            ano,
            disponivel: true,
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProdutoFiltro {
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub ano: Option<u16>,
    pub cor: Option<String>,
    pub disponivel: Option<bool>,
}
