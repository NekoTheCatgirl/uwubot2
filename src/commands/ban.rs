use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::BANNED;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    interaction.create_response(
        &ctx.http,
        CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())
    ).await?;
    let mut banned = BANNED.lock().await;
    let response = if banned.channels.clone().contains(&interaction.channel_id.get()) == false {
        banned.channels.push(interaction.channel_id.get());
        let _ = banned.save().await;
        "Got it! Im now banned from this channel!"
    } else {
        "Im already banned from this channel!"
    };
    interaction.edit_response(&ctx.http, EditInteractionResponse::new().content(response)).await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ban")
        .description("Blocks the bot from sending uwu in this channel")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .dm_permission(false)
}
