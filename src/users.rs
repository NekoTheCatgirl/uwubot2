use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};
use std::fs;

/// Track user data. Only if the user consented by giving Birthday info.

#[derive(Serialize, Deserialize)]
pub struct UserDatabase {
    pub users: Vec<UserData>
}

const USERS_DATABASE: &str = "users.json";

impl UserDatabase {
    pub fn load() -> UserDatabase {
        if let Ok(contents) = fs::read_to_string(USERS_DATABASE) {
            if let Ok(banned) = serde_json::from_str(&contents) {
                banned
            } else {
                UserDatabase {
                    users: Vec::new(),
                }
            }
        } else {
            UserDatabase {
                users: Vec::new(),
            }
        }
    }

    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string(self)?;
        let mut file = File::create(USERS_DATABASE).await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub uuid: u64,
    pub likes_uwu: bool,
    pub birthday: DateTime<Utc>,
}