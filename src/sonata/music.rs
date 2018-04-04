use serde_json;
use std::process::Command;
use serenity::voice;
use serenity::voice::Handler;
use std::path::Path;
use crossbeam_deque::Deque;
use serenity::model::id::{UserId, ChannelId, GuildId};
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MusicItem {
    pub uploader_id: String,
    pub webpage_url: String,
    pub id: String,
    pub duration: i64,
    pub fulltitle: String,
    pub like_count: i64,
    pub view_count: i64,
    pub title: String,
    pub uploader: String,
    pub dislike_count: i64,
    pub extractor: String,
    pub average_rating: f64,
    pub thumbnail: String,
    pub uploader_url: String
}

impl MusicItem {
    pub fn new(data_json_string: String) -> MusicItem {
        let music_data: MusicItem = serde_json::from_str(&data_json_string)
        .expect("Failed to parse music info JSON.");
        return music_data;
    }
    pub fn download(&self) {
        let audio_path = format!("cache/{}.mp3", &self.id);
        let downloaded = Path::new(&audio_path).exists();
        if !downloaded {
            let _response = Command::new("youtube-dl")
            .args(&[
                "--extract-audio",
                "--audio-format=mp3",
                "--audio-quality=0",
                "--output=cache/%(id)s.%(ext)s",
                &self.webpage_url
            ])
            .output()
            .expect("Failed to download music item!");
        }
    }
    pub fn play(&self, handler: &mut Handler) {
        self.download();
        let audio_path = format!("cache/{}.mp3", &self.id);
        let source = voice::ffmpeg(&audio_path).unwrap();
        handler.play(source);
    }
}

pub struct QueueItem {
    _req_uid: UserId,
    _req_gid: GuildId,
    _req_cid: ChannelId,
    _music_item: MusicItem
}

pub struct MusicCore {
    _queues: HashMap<GuildId, Deque<QueueItem>>
}

impl MusicCore {
    pub fn extract_data(url: String) -> MusicItem {
        let response = Command::new("youtube-dl")
        .args(&[
            "--skip-download",
            "--print-json",
            &url
        ])
        .output().expect("Failed to start the data extraction subprocess");
        let info_json = String::from_utf8_lossy(&response.stdout).to_string();
        return MusicItem::new(info_json);
    }
}