// src/youtube.rs

use crate::models::Video;
use iso8601_duration::Duration;
use reqwest::Response;
use serde_json::{Value, to_string};

/// Retorna uma tupla com o título da playlist e a quantidade de vídeos
pub fn fetch_playlist_metadata(playlist_id: &str, api_token: &str) -> (String, u64) {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/playlists?part=contentDetails,snippet&id={playlist_id}&key={api_token}"
    );

    let response = reqwest::blocking::get(url).expect("Erro no request 1");

    let body = response
        .text()
        .expect("Erro ao ler body do primeiro request");

    let json: Value = serde_json::from_str(&body).expect("Erro ao transformar body em json");

    let title = json["items"][0]["snippet"]["title"]
        .as_str()
        .expect("Erro ao obter título da playlist")
        .to_string();

    let count = json["items"][0]["contentDetails"]["itemCount"]
        .as_u64()
        .expect("Não foi possível verificar a quantidade de vídeos na playlist");

    (title, count)
}

/// Retorna um vetor com os IDs de todos os vídeos de uma playlist
pub fn fetch_video_ids(playlist_id: &str, api_token: &str) -> Vec<String> {
    let mut videos_ids: Vec<String> = Vec::new();

    let url = format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=contentDetails&playlistId={playlist_id}&maxResults=50&key={api_token}"
    );

    let response = reqwest::blocking::get(url).expect("Erro ao fazer segundo request");

    let body = response
        .text()
        .expect("Erro ao ler body do segundo request");

    let mut json: Value = serde_json::from_str(&body).expect("Erro ao transformar body em json");

    let items = json["items"].as_array().expect("\"items\" não é array");
    for item in items {
        let video_id = item["contentDetails"]["videoId"]
            .as_str()
            .expect("Erro ao extrair ID do vídeo");
        videos_ids.push(video_id.to_string());
    }

    while let Some(page_token) = json["nextPageToken"].as_str() {
        let response = reqwest::blocking::get(format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=contentDetails&playlistId={playlist_id}&maxResults=50&pageToken={page_token}&key={api_token}"
    )).expect("Erro ao fazer segundo request");

        let body = response
            .text()
            .expect("Erro ao ler body do segundo request");

        json = serde_json::from_str(&body).expect("Erro ao transformar body em json");

        let items = json["items"].as_array().expect("\"items\" não é array");
        for item in items {
            let video_id = item["contentDetails"]["videoId"]
                .as_str()
                .expect("Erro ao extrair ID do vídeo");
            videos_ids.push(video_id.to_string());
        }
    }
    videos_ids
}

pub fn fetch_video_details(video_id: &str, api_token: &str) -> Video {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=snippet,contentDetails&id={video_id}&key={api_token}"
    );

    let response = reqwest::blocking::get(url).expect("Erro ao fazer terceiro request");

    let body = response
        .text()
        .expect("Erro ao ler body do terceiro request");

    let mut json: Value = serde_json::from_str(&body).expect("Erro ao transformar body em json");

    let title = json["items"][0]["snippet"]["title"]
        .as_str()
        .expect("Erro ao extrair título do vídeo");

    let duration = json["items"][0]["contentDetails"]["duration"]
        .as_str()
        .expect("Não foi possível extrair duração do vídeo");

    let duration_seconds = iso8601_duration::Duration::parse(duration)
        .expect("Não foi possível converter duração")
        .num_seconds()
        .expect("Não foi possível converter duração do vídeo em segundos")
        as u64;

    Video {
        id: video_id.to_string(),
        title: title.to_string(),
        duration_seconds,
    }
}
