use chrono_tz::Tz;
use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
        )
        .await?;
    let options = interaction.data.options();
    let mut date: String;
    let mut tzone: Tz;
    for opt in options {
        match opt {
            ResolvedOption {
                name,
                value: ResolvedValue::String(value),
                ..
            } => match name {
                "date" => {}
                "timezone" => {}
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("birthday")
        .description("Registers your birthday!")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "date",
                "The day you were born (yyyy-mm-dd)",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "timezone",
                "The timezone you live in. (eg: EST, CEST. Alt UTC works)",
            )
            .required(true),
        )
}
