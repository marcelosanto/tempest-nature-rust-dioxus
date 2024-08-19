
# Tempest WeatherApp

## Descrição

**WeatherApp** é um aplicativo simples de previsão do tempo desenvolvido em Rust usando o framework Dioxus para a interface do usuário e consumindo a API da OpenWeather para fornecer dados meteorológicos em tempo real. Este projeto demonstra a construção de uma interface gráfica com Rust e Dioxus, junto com a integração de uma API externa.

## Funcionalidades

-   Exibe a previsão do tempo atual para uma cidade especificada.
-   Mostra informações como temperatura, condição climática, umidade e velocidade do vento.
-   Interface simples e responsiva.
-   Suporte para múltiplas unidades (Celsius e Fahrenheit).

## Capturas de Tela

## Requisitos

-   Rust 1.60+ (com suporte a Cargo)
-   Chave de API da OpenWeather
-   Dioxus CLI (para desenvolvimento local)

## Instalação

### 1. Clone o Repositório

```bash
git clone https://github.com/seu-usuario/weather-app.git cd weather-app
```
### 2. Configure a Chave da API
Crie um arquivo `.env` na raiz do projeto e adicione sua chave da API da OpenWeather:

``env
OPENWEATHER_API_KEY=your_api_key_here
``

### 3. Compile e Execute o Projeto

Você pode compilar e executar o projeto localmente usando o Cargo:
```bash
`cargo run`
````
Ou, se estiver usando a Dioxus CLI para desenvolvimento:
```bash 
`dioxus serve`
```
## Como Usar

1.  Ao iniciar o aplicativo, insira o nome de uma cidade na barra de pesquisa.
2.  Pressione "Enter" ou clique no botão de busca.
3.  As informações meteorológicas da cidade especificada serão exibidas.

## Estrutura do Projeto
```plaintext 
weather-app/ 
├── src/ 
│ ├── main.rs # Ponto de entrada principal do aplicativo 
│ ├── api.rs # Módulo responsável por chamadas à API da OpenWeather 
│ ├── components/ # Componentes da interface do usuário 
│ └── utils.rs # Funções auxiliares (ex.: conversão de unidades) 
├── assets/ 
│ └── screenshot.png # Captura de tela do aplicativo 
├── .env # Arquivo de configuração da chave de API 
├── Cargo.toml # Configurações de dependências do Cargo 
└── README.md # Documentação do projeto
```

## Tecnologias Utilizadas

-   **[Rust](https://www.rust-lang.org/pt-BR):** Linguagem de programação principal.
-   **[Dioxus](https://dioxuslabs.com/):** Framework para construir interfaces de usuário.
-   **[OpenWeather API](https://api.openweathermap.org):** Fonte dos dados meteorológicos.
-   **[Reqwest](https://docs.rs/reqwest/latest/reqwest/):** Cliente HTTP para fazer requisições à API.
-   **[Serde](https://docs.rs/serde_json/latest/serde_json/):** Para serialização/deserialização de JSON.
- **_[Tailwind_ CSS](https://tailwindcss.com/)** CSS framework for rapidly building modern websites without ever leaving your HTML

## Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues e pull requests para melhorias ou correções.

1.  Fork o projeto
2.  Crie sua feature branch (`git checkout -b feature/nova-feature`)
3.  Commit suas mudanças (`git commit -m 'Adiciona nova feature'`)
4.  Push para a branch (`git push origin feature/nova-feature`)
5.  Abra um Pull Request

## Licença

Este projeto é licenciado sob a Licença MIT - veja o arquivo LICENSE para mais detalhes.


# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload --platform desktop
```