use std::sync::Arc;

use aluguel_de_carros_api::{
    AppState,
    entities::{self, Produto},
    middleware, routes,
};
use axum::{
    Router,
    routing::{any, delete, get, post, put},
};
use sha2::Digest;
use tokio::{self, sync::RwLock};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type StdError = Box<dyn std::error::Error>;

const IP_ADDR: &str = "0.0.0.0:9876";

#[tokio::main]
async fn main() -> Result<(), StdError> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aluguel_de_carros_api=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Iniciando o servidor...");

    let shared_state = Arc::new(RwLock::new(AppState::default()));

    shared_state.write().await.usuarios.push(entities::Usuario {
        id: 1,
        nome: "Admin".to_string(),
        telefone: "123456789".to_string(),
        email: "admin@example.com".to_string(),
        pwd_hash: hex::encode(sha2::Sha256::digest("admin123".as_bytes())),
        role: entities::Role::Administrador,
    });

    shared_state.write().await.produtos = mock_produtos();

    let public_routes = Router::new()
        .route("/", any(routes::root))
        .route("/login", post(routes::login))
        .route("/register", post(routes::register))
        .fallback(routes::not_found);

    let protected_routes = Router::new()
        .route("/protected_test", get(|| async { "Rota protegida" }))
        .route("/produtos", get(routes::produtos_list))
        .route("/produtos/{sku}", get(routes::produto_sku))
        .route("/produtos/{sku}/alugar", post(routes::alugar_produto))
        .route("/produtos/{sku}/devolver", post(routes::devolver_produto))
        .route("/usuario", delete(routes::delete_account))
        .route_layer(axum::middleware::from_fn(middleware::auth_middleware));

    let protected_adm_routes = Router::new()
        .route("/admin_test", get(|| async { "Rota admin protegida" }))
        .route("/produtos", post(routes::produto_create))
        .route("/produtos/{sku}/delete", delete(routes::produto_delete))
        .route("/produtos/{sku}/update", put(routes::produto_update))
        .route_layer(axum::middleware::from_fn(
            middleware::auth_adm_middleware,
        ));

    info!("Rotas criadas.");

    let api = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(protected_adm_routes)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(IP_ADDR).await?;
    info!("Servidor rodando em {}...", listener.local_addr()?);

    axum::serve(listener, api).await?;

    Ok(())
}

/// Mock de produtos feito pelo Gemini, inspirado em carros
/// do jogo Need for Speed: Most Wanted (2005) e outros
/// carros icônicos.
fn mock_produtos() -> Vec<Produto> {
    vec![
        // O ícone absoluto
        Produto::new("BMW", "M3 GTR (E46)", 2001, "Prata e Azul"),
        // Carros da Blacklist (Cores dos Rivais)
        Produto::new("Mercedes-Benz", "SLR McLaren", 2005, "Preto"), /* Bull (#2) */
        Produto::new("Aston Martin", "DB9", 2005, "Prata"), // Ronnie (#3)
        Produto::new("Dodge", "Viper SRT10", 2005, "Verde"), // JV (#4)
        Produto::new("Chevrolet", "Corvette C6", 2005, "Laranja"), /* Webster (#5) */
        Produto::new("Lamborghini", "Gallardo", 2005, "Prata"),    // Ming (#6)
        Produto::new("Porsche", "Cayman S", 2006, "Preto"), // Baron (#10)
        Produto::new("Mitsubishi", "Lancer Evolution VIII", 2004, "Azul"), /* Earl (#9) */
        Produto::new("Ford", "Mustang GT", 2005, "Verde Escuro"), /* Jewels (#8) */
        Produto::new("Toyota", "Supra", 1998, "Vermelho"),        // Vic (#13)
        Produto::new("Mazda", "RX-7", 1995, "Preto"),             // Izzy (#12)
        Produto::new("Mitsubishi", "Eclipse", 1999, "Preto"), // Big Lou (#11)
        Produto::new("Volkswagen", "Golf GTI", 2005, "Preto"), // Sonny (#15)
        Produto::new("Lexus", "IS 300", 2004, "Amarelo"),     // Taz (#14)
        // Exóticos de alto desempenho
        Produto::new("Porsche", "Carrera GT", 2004, "Prata"),
        Produto::new("Lamborghini", "Murciélago", 2004, "Amarelo"),
        Produto::new("Ford", "GT", 2005, "Branco com listras azuis"),
        Produto::new("Lotus", "Elise", 2004, "Laranja"),
        // Iniciais e Tuners clássicos
        Produto::new("Chevrolet", "Cobalt SS", 2005, "Azul"),
        Produto::new("Fiat", "Punto", 2005, "Prata"),
        Produto::new("Mazda", "RX-8", 2004, "Vermelho"), // Carro da Mia
        Produto::new("Audi", "TT 3.2 Quattro", 2004, "Branco"),
        Produto::new("Audi", "A3 3.2 Quattro", 2005, "Cinza"),
    ]
}
