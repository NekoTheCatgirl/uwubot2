use tokio::{ fs::File, io::AsyncWriteExt };

use std::fs;
use serde::{ Deserialize, Serialize };

const BANNED_FILE: &str = "banned.json";

#[derive(Serialize, Deserialize)]
pub struct BannedChannels {
    pub channels: Vec<u64>,
}

impl BannedChannels {
    pub fn load() -> BannedChannels {
        if let Ok(contents) = fs::read_to_string(BANNED_FILE) {
            if let Ok(banned) = serde_json::from_str(&contents) {
                banned
            } else {
                BannedChannels {
                    channels: Vec::new(),
                }
            }
        } else {
            BannedChannels {
                channels: Vec::new(),
            }
        }
    }

    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string(self)?;
        let mut file = File::create(BANNED_FILE).await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }
}
