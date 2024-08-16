use chrono::{TimeZone, Utc};
use rand::Rng;
use serenity::all::{Context, Message};

use crate::BANNED;

/// Checks if the message function should trigger.
pub async fn trigger_check(rng: &mut impl Rng, ctx: &Context, message: &Message) -> bool {
    if rng.gen_range(0..=100) <= 5
        && BANNED
            .lock()
            .await
            .channels
            .contains(&message.channel_id.get())
            == false
    {
        return true;
    }
    false
}
