use rand::Rng;
use serenity::all::Message;

use crate::{matter::MatterTrait, BANNED};

/// Checks if the message function should trigger.
pub async fn trigger_check(rng: &mut impl Rng, matter: &impl MatterTrait, message: &Message) -> bool {
    if rng.gen_range(0..=100) <= matter.get_chance()
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
