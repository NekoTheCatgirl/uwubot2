use std::collections::HashMap;

use tokio::{fs::File, io::AsyncWriteExt};

use serde::{Deserialize, Serialize};
use std::fs;

const LEADER_FILE: &str = "leaderboard.json";

#[derive(Serialize, Deserialize)]
pub struct Leaderboard {
    pub total_uwu: u128,
    pub leaderboard: HashMap<u64, u32>,
}

impl Leaderboard {
    pub fn load() -> Leaderboard {
        if let Ok(contents) = fs::read_to_string(LEADER_FILE) {
            if let Ok(leaderboard) = serde_json::from_str(&contents) {
                leaderboard
            } else {
                Leaderboard {
                    total_uwu: 0,
                    leaderboard: HashMap::new(),
                }
            }
        } else {
            Leaderboard {
                total_uwu: 0,
                leaderboard: HashMap::new(),
            }
        }
    }

    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string(self)?;
        let mut file = File::create(LEADER_FILE).await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }

    pub fn get_top_10(&self) -> (u128, Vec<(u64, u32)>) {
        let mut map: Vec<_> = self.leaderboard.clone().into_iter().collect();
        map.sort_by(|a, b| b.1.cmp(&a.1));
        let values = map.into_iter().take(10).collect();
        (self.total_uwu.clone(), values)
    }
}
