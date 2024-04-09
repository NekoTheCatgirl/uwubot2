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
    let response = if banned.channels.clone().contains(&interaction.channel_id.get()) {
        let index = banned.channels
            .clone()
            .into_iter()
            .position(|i| i == interaction.channel_id.get())
            .unwrap();
        banned.channels.remove(index);
        let _ = banned.save().await;
        "I am now allowed to speak in here again!"
    } else {
        "I am already allowed to speak in here!"
    };
    interaction.edit_response(&ctx.http, EditInteractionResponse::new().content(response)).await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("unban")
        .description("Allows the bot to uwu this channel")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .dm_permission(false)
}
