// src/models.rs

#[derive(Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub duration_seconds: usize,
}

#[derive(Debug)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub videos_count: usize,
    pub videos: Vec<Video>,
    pub duration_seconds: usize,
}
