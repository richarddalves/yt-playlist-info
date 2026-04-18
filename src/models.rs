// src/models.rs

#[derive(Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub duration_seconds: u64,
}

#[derive(Debug)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub videos_count: u64,
    pub videos: Vec<Video>,
    pub duration_seconds: u64,
}

impl Playlist {
    pub fn new(id: String, title: String, videos_count: u64) -> Self {
        Self {
            id,
            title,
            videos_count,
            videos: Vec::new(),
            duration_seconds: 0,
        }
    }

    pub fn add_video(&mut self, video: Video) {
        self.videos.push(video);
    }
}
