// src/main.rs

#![allow(unused)] // temporario, remover antes do 1.0

mod models;
mod youtube;

use models::{Playlist, Video};

use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let api_token = dotenvy::var("YTB_API_TOKEN")
        .expect("A variável de ambiente `YTB_API_TOKEN` não foi encontrada.");

    let playlist_id = ask_for_playlist_id("Insira o ID da Playlist: ");

    let start_time = Instant::now();

    // Passo 1: request para /playlist para obter nome da playlist e quantidade de vídeos
    let (playlist_title, videos_count) = youtube::fetch_playlist_metadata(&playlist_id, &api_token);
    let mut playlist = Playlist::new(playlist_id, playlist_title, videos_count as u64);

    // Passo 2: request (ou loop) para /playlistItems para obter o id de cada vídeo
    let videos_ids = youtube::fetch_video_ids(&playlist.id, &api_token);

    // Passo 3: request para /videos para obter titulo e duração de cada vídeo
    // obs.: está ineficiente, estou fazendo um requet novo para cada video.
    // se a playlist tiver 1000 videos, serão 1000 requisições. a api suporta mais
    // do que um videoId por request, mas tratarei depois

    youtube::fetch_video_details(&videos_ids, &mut playlist, &api_token);

    // agora basta somar o tempo total de cada video e exibir
    let mut sum_duration = 0;

    for video in &playlist.videos {
        sum_duration += video.duration_seconds;
    }

    let duration_string = format_duration(sum_duration);
    let end_time = start_time.elapsed();

    println!("Resultados encontrados em {:.3?}\n", end_time);

    println!("Nome: {}", &playlist.title);
    println!("Quantidade de videos: {}", &playlist.videos_count);
    println!("Duração total: {}", duration_string);

    println!();

    for (i, video) in playlist.videos.iter().enumerate() {
        println!("{}) {}", i + 1, video.title);
    }

    println!("\nTempo total: {:.3?}", start_time.elapsed());
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

fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}
