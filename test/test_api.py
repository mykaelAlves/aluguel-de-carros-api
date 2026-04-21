# Testes feito pelo Gemini

import requests

# Atualizado para a porta correta do seu main.rs
BASE_URL = "http://127.0.0.1:9876"

def run_tests():
    print("=== Iniciando Testes da API Rockport City ===\n")

    # 1. Login com o usuário Admin preenchido no state
    print("1. Testando POST /login ...")
    login_payload = {
        "email": "admin@example.com",
        "password": "admin123"
    }
    
    res = requests.post(f"{BASE_URL}/login", json=login_payload)
    print(f"Status: {res.status_code}")
    
    if res.status_code != 200:
        print(f"Erro no login: {res.text}")
        return

    # Pega o token (assumindo que a API retorna o token direto em texto ou em um JSON)
    # Se a sua API retornar JSON tipo {"token": "seu_token"}, use: token = res.json().get("token")
    token = res.text.replace('"', '').strip() 
    print(f"Login efetuado! Token recebido.\n")

    # Criar uma sessão para enviar o Authorization Header automaticamente em todas as rotas protegidas
    session = requests.Session()
    session.headers.update({"Authorization": f"Bearer {token}"})

    # 2. Listar todos os carros (Rota Protegida)
    print("2. Testando GET /produtos (Sem filtros) ...")
    res = session.get(f"{BASE_URL}/produtos")
    print(f"Status: {res.status_code}")
    
    produtos = []
    if res.status_code == 200:
        produtos = res.json()
        print(f"Carros encontrados: {len(produtos)}\n")
    else:
        print(f"Falha ao buscar produtos: {res.text}\n")
        return

    # Vamos gerar um SKU fake para testar usando o nome da marca e modelo
    # Ex: "BMW-M3 GTR (E46)"
    # Pegamos os dados reais retornados pela API
    carro_teste = produtos[0]
    marca = carro_teste.get("marca", "")
    modelo = carro_teste.get("modelo", "")
    ano = carro_teste.get("ano", 0)
    cor = carro_teste.get("cor", "")

    # Replicando a exata lógica do Produto::new do Rust
    filtered_marca = marca.replace(" ", "")
    filtered_modelo = modelo.replace(" ", "")
    
    sku_teste = f"{filtered_marca}_{filtered_modelo}_{ano}_{cor}"

    print(f"=== Iniciando testes de Locação para o veículo: {marca} {modelo} (SKU: {sku_teste}) ===\n")

    # 3. Alugar o carro
    print(f"3. Testando POST /produtos/{sku_teste}/alugar ...")
    res = session.post(f"{BASE_URL}/produtos/{sku_teste}/alugar")
    print(f"Status: {res.status_code} (Esperado: 200)\n")

    # 4. Tentar alugar novamente (Conflito)
    print(f"4. Testando POST /produtos/{sku_teste}/alugar NOVAMENTE (Forçando erro) ...")
    res = session.post(f"{BASE_URL}/produtos/{sku_teste}/alugar")
    print(f"Status: {res.status_code} (Esperado: 409)\n")

    # 5. Devolver o carro
    print(f"5. Testando POST /produtos/{sku_teste}/devolver ...")
    res = session.post(f"{BASE_URL}/produtos/{sku_teste}/devolver")
    print(f"Status: {res.status_code} (Esperado: 200)\n")

    print("=== Testes Finalizados ===")

if __name__ == "__main__":
    try:
        run_tests()
    except requests.exceptions.ConnectionError:
        print(f"ERRO: Não foi possível conectar a {BASE_URL}.")