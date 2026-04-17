use serde::{Deserialize, Serialize};
use std::io::{self, Write};


fn main() {
    let api_token = dotenvy::var("YTB_API_TOKEN").expect("A variável de ambiente `YTB_API_TOKEN` não foi encontrada.");


}

fn ask_for_playlist_id(msg: &str) -> String {

    let mut buf = String::new();
    print!("{msg}");
    io::stdout().flush().expect("Erro ao fazer flush");

    io::stdin().read_line(&mut buf).expect("Erro ao ler input");

    buf.trim().to_string()

    // por enquanto apenas suponho que o ID está correto
    // TODO: futuramente tratar o ID e retornar Result em vez de String
}
