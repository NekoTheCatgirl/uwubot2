use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::LEADERBOARD;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    interaction.create_response(
        &ctx.http,
        CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())
    ).await?;
    let options = interaction.data.options();
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = options.first() {
        let leaderboard = LEADERBOARD.lock().await;
        let count = leaderboard.leaderboard.get(&user.id.get());
        let embed = if let Some(count) = count {
            let username = user.name.clone();
            CreateEmbed::new()
                .title(format!("{username}")) 
                .description(format!("Has been uwu'd {count} times"))
                .footer(CreateEmbedFooter::new("UwU bot, provided to you by Neko"))
                .color(Colour::FABLED_PINK)
        } else {
            CreateEmbed::new()
                .title("Not found")
                .description("That user has yet to be cursed by me!")
                .footer(CreateEmbedFooter::new("UwU bot, provided to you by Neko"))
                .color(Colour::FABLED_PINK)
        };
        interaction.edit_response(
            &ctx.http,
            EditInteractionResponse::new().add_embed(embed)
        ).await?;
    } else {
        let (total, top) = LEADERBOARD.lock().await.get_top_10();
        let mut fields_vec = Vec::new();
        for (user, count) in top {
            let username = {
                if let Ok(user) = ctx.http.get_user(user.into()).await {
                    user.name
                } else {
                    "unknown_user".into()
                }
            };
            fields_vec.push((format!("{username}"), format!("Has been uwu'd {count} times"), false));
        }
        let embed = CreateEmbed::new()
            .title("Leaderboard")
            .description(format!("Total uwu's delivered {total}"))
            .fields(fields_vec)
            .footer(CreateEmbedFooter::new("UwU bot, provided to you by Neko"))
            .color(Colour::FABLED_PINK);
        interaction.edit_response(
            &ctx.http,
            EditInteractionResponse::new().add_embed(embed)
        ).await?;
    }
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leaderboard")
        .description("Shows the leaderboard")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "target",
                "The target you want to get the count of"
            ).required(false)
        )
}
