use chrono::{Datelike, Local, TimeZone, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

use crate::USER_DATABASE;

const MATTER_FILE: &str = "matter.json";

pub async fn get_theme_based_on_date(uuid: u64) -> &'static str {
    let now_utc = Utc::now();
    let now_local = Local::now();
    let month = now_local.month();
    let day = now_local.day();

    // Fetch user data
    let user_database = USER_DATABASE.lock().await;
    let user_data = user_database.users.iter().find(|user| user.uuid == uuid);

    if let Some(user) = user_data {

        let user_tz = user.birthday.timezone();
        let user_local_time = user_tz.from_utc_datetime(&now_utc.naive_utc());
        if user_local_time.month() == month && user_local_time.day() == day {
            return "birthday";
        }
    }

    match (month, day) {
        (12, 24..=31) => "christmas",
        (10, 31) => "halloween",
        (2, 14) => "valentines",
        (7, 4) => "fourth_of_july",
        (3, 17) => "st_patricks",
        (9..=11, _) => "autumn",
        (6..=8, _) => "summer",
        (12 | 1 | 2, _) => "winter",
        _ => "normal",
    }
}

#[derive(Serialize, Deserialize)]
pub struct MatterDict {
    dict: HashMap<String, Matter>,
}

impl MatterDict {
    pub async fn load() -> Result<MatterDict, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(MATTER_FILE).await?;
        let dict = serde_json::from_str(&contents)?;
        Ok(dict)
    }

    pub fn get(&self, name: impl Into<String>) -> Option<&impl MatterTrait> {
        self.dict.get(&name.into())
    }
}

#[derive(Serialize, Deserialize)]
struct Matter {
    base_chance: u8,
    long_form: Vec<String>,
    permutation: MatterPermutation,
}

pub trait MatterTrait {
    fn get_chance(&self) -> u8;

    fn get_long(&self, rng_source: &mut impl Rng) -> String;

    fn gen_permutation(&self, rng_source: &mut impl Rng) -> String;
}

impl MatterTrait for Matter {
    fn get_chance(&self) -> u8 {
        self.base_chance
    }

    fn get_long(&self, rng_source: &mut impl Rng) -> String {
        self.long_form[rng_source.gen_range(0..self.long_form.len())].clone()
    }

    fn gen_permutation(&self, rng_source: &mut impl Rng) -> String {
        self.permutation.gen_permutation(rng_source)
    }
}

#[derive(Serialize, Deserialize)]
struct MatterPermutation {
    inner: Vec<char>,
    outer: Vec<char>,
}

impl MatterPermutation {
    fn gen_permutation(&self, rng_source: &mut impl Rng) -> String {
        let mut val = String::new();
        val.push(self.outer[rng_source.gen_range(0..self.outer.len())]);
        val.push(self.inner[rng_source.gen_range(0..self.inner.len())]);
        val.push(self.outer[rng_source.gen_range(0..self.outer.len())]);
        val
    }
}
