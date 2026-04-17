// src/main.rs

#![allow(unused)] // temporario, remover antes do 1.0

mod models;
mod youtube;

use models::{Playlist, Video};

use std::io::{self, Write};

fn main() {
    let api_token = dotenvy::var("YTB_API_TOKEN")
        .expect("A variável de ambiente `YTB_API_TOKEN` não foi encontrada.");

    //ask_for_playlist_id("Insira o ID da Playlist: ");
    let playlist_id = "PLvMVt4c68EeXRV2R1fdieei9HwpuUKXKL"; // "PLgCQgdu45p-xwQUjZygBS2NKDLB46cU6O"; // hardcoded por enquanto para facilitar

    // Passo 1: request para /playlist para obter nome da playlist e quantidade de vídeos
    let (playlist_title, videos_count) = youtube::fetch_playlist_metadata(playlist_id, &api_token);

    // Passo 2: request (ou loop) para /playlistItems para obter o id de cada vídeo
    let videos_ids = youtube::fetch_video_ids(playlist_id, &api_token)

    // Passo 3: request para /videos para obter titulo e duração de cada vídeo
    // for id in videos_ids {
    //     let response = reqwest::blocking::get(format!(
    //         "https://www.googleapis.com/youtube/v3/videos?part=contentDetails,snippet&id="
    //     ))
    //     .expect("Erro ao fazer terceiro request");

    //     let body = response.text().expect("Erro ao transformar body em json");

    //     let json: Value = serde_json::from_str(&body).expect("Erro ao transformar body em json");
    // }
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
