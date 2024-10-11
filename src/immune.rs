use tokio::{fs::File, io::AsyncWriteExt};

use serde::{Deserialize, Serialize};
use std::fs;

const IMMUNE_FILE: &str = "immune.json";

#[derive(Serialize, Deserialize)]
pub struct ImmuneUsers {
    pub users: Vec<u64>,
}

impl ImmuneUsers {
    pub fn load() -> ImmuneUsers {
        if let Ok(contents) = fs::read_to_string(IMMUNE_FILE) {
            if let Ok(banned) = serde_json::from_str(&contents) {
                banned
            } else {
                ImmuneUsers {
                    users: Vec::new(),
                }
            }
        } else {
            ImmuneUsers {
                users: Vec::new(),
            }
        }
    }

    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string(self)?;
        let mut file = File::create(IMMUNE_FILE).await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }
}
