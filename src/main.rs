use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use serde_json::Value;


fn main() {
    let api_token = dotenvy::var("YTB_API_TOKEN").expect("A variável de ambiente `YTB_API_TOKEN` não foi encontrada.");

    //ask_for_playlist_id("Insira o ID da Playlist: ");
    let playlist_id = "PLHg022HMFzFCRq-5ZVR3hiiCkGPJ3Ur1D"; // hardcoded por enquanto para facilitar

    // Passo 1: request para /playlist para obter nome da playlist e quantidade de vídeos
    let response = reqwest::blocking::get(format!("https://www.googleapis.com/youtube/v3/playlists?part=contentDetails,snippet&id={playlist_id}&key={api_token}")).expect("Erro no request 1");

    let body = response.text().expect("Erro ao ler body do primeiro request");

    let json: Value = serde_json::from_str(&body).expect("Erro ao transformar body em json");
        
    let playlist_title = json["items"][0]["snippet"]["title"].as_str().expect("Erro ao obter título da playlist");
    
    let videos_count = json["items"][0]["contentDetails"]["itemCount"].as_u64().expect("Não foi possível verificar a quantidade de vídeos na playlist");

    println!("A playlist {playlist_title} tem {videos_count} videos."); // apenas para debug

    // Passo 2: request (ou loop) para /playlistItems para obter o id de cada vídeo

    // Passo 3: request para /videos para obter titulo e duração de cada vídeo
}

fn _ask_for_playlist_id(msg: &str) -> String {

    let mut buf = String::new();
    print!("{msg}");
    io::stdout().flush().expect("Erro ao fazer flush");

    io::stdin().read_line(&mut buf).expect("Erro ao ler input");

    buf.trim().to_string()

    // por enquanto apenas suponho que o ID está correto
    // TODO: futuramente tratar o ID e retornar Result em vez de String
}
