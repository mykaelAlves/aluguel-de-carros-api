**(README foi gerado pelo Gemini)**

# Aluguel de Carros API

Uma API RESTful desenvolvida em **Rust** com o framework **Axum** para a gestão de aluguer de veículos. Este projeto serve como um sistema base com gestão de utilizadores (clientes e administradores), catálogo de produtos (veículos) e funcionalidades de aluguer/devolução.

Curiosidade: O catálogo inicial de veículos é inspirado nos carros icónicos do jogo *Need for Speed: Most Wanted (2005)*.

## 🚀 Funcionalidades

* **Autenticação e Autorização:** Controlo de acessos baseado em *Tokens* e papéis de utilizador (`Cliente` e `Administrador`). Hash de palavras-passe com `SHA-256`.
* **Gestão de Utilizadores:**
    * Registo de novos clientes.
    * Autenticação (Login) para obtenção de Token.
    * Remoção de conta.
* **Gestão de Frota (Produtos):**
    * Listagem de veículos com filtros (marca, modelo, ano, cor, disponibilidade).
    * Consulta detalhada por *SKU*.
    * Adição, atualização e remoção de veículos (exclusivo para Administradores).
* **Locação:**
    * Alugar um veículo disponível.
    * Devolver um veículo alugado.
* **Documentação OpenAPI/Swagger:** Integração configurada via `utoipa` e `utoipa-swagger-ui`.

## 🛠️ Tecnologias Utilizadas

* **Linguagem:** [Rust](https://www.rust-lang.org/) (Edição 2024)
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Assincronismo:** Tokio
* **Serialização:** Serde & Serde JSON
* **Criptografia (Hashing):** SHA2 e Hex
* **Utilitários:** Chrono (Gestão de datas/horas), Tracing (Logs)
* **Documentação:** Utoipa

## 📦 Como Executar

### Pré-requisitos
* [Rust e Cargo](https://rustup.rs/) instalados no sistema.

### Executar a API

1.  Clone o repositório ou navegue até à pasta raiz do projeto.
2.  Execute o comando cargo para compilar e iniciar o servidor:
    ```bash
    cargo run
    ```
3. A API estará disponível no endereço: `http://0.0.0.0:9876`

## 🚏 Rotas da API

### Públicas
* `POST /login` - Autenticação de utilizadores. Retorna o Token de acesso.
* `POST /register` - Registo de um novo cliente.

### Protegidas (Requer Token de Cliente ou Admin)
* `GET /produtos` - Lista todos os veículos (suporta *query params* para filtros).
* `GET /produtos/{sku}` - Devolve os detalhes de um veículo específico pelo seu SKU.
* `POST /produtos/{sku}/alugar` - Regista o aluguer de um veículo.
* `POST /produtos/{sku}/devolver` - Regista a devolução de um veículo.
* `DELETE /usuario` - Remove a conta do utilizador (suporta *query param* `nome`).

### Protegidas (Exclusivo para Administradores)
* `POST /produtos` - Adiciona um novo veículo à frota.
* `PUT /produtos/{sku}/update` - Atualiza as informações de um veículo existente.
* `DELETE /produtos/{sku}/delete` - Remove um veículo da frota.

*(Existem também as rotas `/protected_test` e `/admin_test` para testes rápidos de autorização).*

## 🧪 Testes

O projeto inclui um guião de testes em Python localizado na pasta `test/`.

1.  Navegue até a diretoria de testes:
    ```bash
    cd test
    ```
2.  Instale as dependências requeridas:
    ```bash
    pip install -r requirements.txt
    ```
3.  Execute o script com a API a correr num outro terminal:
    ```bash
    python test_api.py
    ```

O script testará automaticamente o login do Administrador, a listagem de produtos, a tentativa de aluguer, o tratamento de conflitos (alugar um veículo já ocupado) e a respetiva devolução.

## 📄 Licença

Este projeto está licenciado sob a Licença MIT. Consulte o ficheiro [LICENSE](LICENSE) para mais detalhes.
