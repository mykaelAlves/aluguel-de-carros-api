use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::{EXPECTED_ADMIN_TOKEN, EXPECTED_USER_TOKEN};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Cliente,
    Administrador,
}

impl Role {
    pub fn to_token(&self) -> &'static str {
        match self {
            Role::Cliente => EXPECTED_USER_TOKEN,
            Role::Administrador => EXPECTED_ADMIN_TOKEN,
        }
    }
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

pub struct Locacao {
    pub usuario_id: u32,
    pub produto_sku: SKU,
    pub tem_multa: bool,
    pub valor_multa: f64,
    pub data_locacao: DateTime<FixedOffset>,
    pub data_devolucao: DateTime<FixedOffset>,
}

impl Locacao {
    const TIMEZONE_OFFSET: i32 = -3 * 3600; // UTC-3 (BRT)

    pub fn new(usuario_id: u32, produto_sku: SKU) -> Self {
        let tz = FixedOffset::east_opt(Self::TIMEZONE_OFFSET).unwrap();
        let now = Utc::now().with_timezone(&tz);
        Locacao {
            usuario_id,
            produto_sku,
            tem_multa: false,
            valor_multa: 0.0,
            data_locacao: now,
            data_devolucao: now + chrono::Duration::days(7),
        }
    }
}

impl Produto {
    pub fn new(marca: &str, modelo: &str, ano: u16, cor: &str) -> Self {
        let filtered_marca = marca.replace(" ", "");
        let filtered_modelo = modelo.replace(" ", "");

        let sku =
            format!("{}_{}_{}_{}", filtered_marca, filtered_modelo, ano, cor);
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
