# Youtube Playlist Info

Programa para extrair duração total de uma playlist e enumerar os seus vídeos na ordem em que foram colocados originalmente.

![Screenshot do terminal](assets/screenshot-cli.avif)

## Pré-requisitos

Você precisa ter **Rust instalado** na sua máquina. Caso você ainda não tenha, a instação pode ser feita seguindo o [tutorial da página oficial do Rust](https://rust-lang.org/pt-BR/learn/get-started/) ou rodando o comando abaixo no terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Instalação

1. Clone esse repositório

```bash
git clone https://github.com/richarddalves/yt-playlist-info.git
```

ou

```bash
git clone git@github.com:richarddalves/yt-playlist-info.git
```

2) Entre na pasta

```bash
cd yt-playlist-info.git
```

3) Crie um arquivo `.env` e crie a variável `YTB_API_TOKEN`.

O token pode ser obtido através do Google Cloud Console para a **YouTube Data API v3**.

*Nota: É esperado que o usuário saiba como obter o token*

4) Rode `cargo run`:

```bash
cargo run
```

O projeto começará a compilar e depois perguntará pelo ID da playlist. Você pode obter o ID a partir da URL.

*Nota: É esperado que o usuário saiba como obter o ID de uma playlist.*

5) Após colar o ID da playlist, o resultado aparecerá:

```bash
Insira o ID da Playlist: PL8iN9FQ7_jt5L1c-bXpix5v0ValjahZw5
Resultados encontrados em 1.289s

Nome: Estrutura de Dados em Linguagem C | Lista Dinâmica Encadeada
Quantidade de videos: 7
Duração total: 52:02

1) Estrutura de Dados em C | Aula 03 - Listas: Definição
2) Estrutura de Dados em C | Aula 10 - Lista Dinâmica Encadeada
3) Estrutura de Dados em C | Aula 11 - Implementação da Lista Dinâmica Encadeada
4) Estrutura de Dados em C | Aula 12 - Informações da Lista Dinâmica Encadeada
5) Estrutura de Dados em C | Aula 13 - Inserção na Lista Dinâmica Encadeada
6) Estrutura de Dados em C | Aula 14 - Remoção da Lista Dinâmica Encadeada
7) Estrutura de Dados em C | Aula 15 - Consulta na Lista Dinâmica Encadeada

Tempo total: 1.289s
```
