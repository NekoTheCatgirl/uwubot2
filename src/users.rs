use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::{fs::File, io::AsyncWriteExt};

/// Track user data. Only if the user consented by giving Birthday info.

#[derive(Serialize, Deserialize)]
pub struct UserDatabase {
    pub users: Vec<UserData>,
}

const USERS_DATABASE: &str = "users.json";

impl UserDatabase {
    pub fn load() -> UserDatabase {
        if let Ok(contents) = fs::read_to_string(USERS_DATABASE) {
            if let Ok(banned) = serde_json::from_str(&contents) {
                banned
            } else {
                UserDatabase { users: Vec::new() }
            }
        } else {
            UserDatabase { users: Vec::new() }
        }
    }
    
    pub fn add_user(&mut self, new_user: UserData) {
        // Check if a user with the same UUID exists
        if let Some(existing_user) = self.users.iter_mut().find(|user| user.uuid == new_user.uuid) {
            // Update the existing user's information
            existing_user.likes_uwu = new_user.likes_uwu;
            existing_user.birthday = new_user.birthday;
        } else {
            // Add the new user if no existing user with the same UUID is found
            self.users.push(new_user);
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
