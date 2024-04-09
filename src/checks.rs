use chrono::{ TimeZone, Utc };
use rand::Rng;
use serenity::all::{ Context, Message };

use crate::BANNED;

/// Checks if the message function should trigger.
pub async fn trigger_check(rng: &mut impl Rng, ctx: &Context, message: &Message) -> bool {
    let current_time = Utc::now();
    let update_trig = Utc.with_ymd_and_hms(2025, 1, 1, 1, 0, 0).earliest().unwrap();
    if current_time > update_trig {
        // Happy new years! The update is now live!
        return true
    } else {
        if
            rng.gen_range(0..=100) <= 5 &&
            BANNED.lock().await.channels.contains(&message.channel_id.get()) == false
        {
            return true;
        }
    }
    false
}
