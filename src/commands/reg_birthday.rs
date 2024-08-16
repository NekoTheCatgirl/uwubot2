use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use log::error;
use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::users::UserData;
use crate::USER_DATABASE;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    // Defer the response
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
        )
        .await?;

    let options = interaction.data.options();

    let mut date: Option<String> = None;
    let mut enjoyer: Option<bool> = None;

    // Extract options
    for opt in options {
        match opt {
            ResolvedOption {
                name,
                value: ResolvedValue::String(value),
                ..
            } => match name {
                "date" => {
                    date = Some(value.to_owned());
                }
                _ => {}
            },
            ResolvedOption {
                name,
                value: ResolvedValue::Boolean(value),
                ..
            } => match name {
                "enjoyer" => {
                    enjoyer = Some(value);
                }
                _ => {}
            },
            _ => {}
        }
    }

    // Validate that all required options are present
    let date = match date {
        Some(d) => d,
        None => {
            if let Err(e) = interaction.edit_response(&ctx.http, EditInteractionResponse::new().content("Something went wrong")).await {
                error!("Something went wrong {e:?}");
            }
            return Ok(())
        },
    };

    let enjoyer = match enjoyer {
        Some(e) => e,
        None => {
            if let Err(e) = interaction.edit_response(&ctx.http, EditInteractionResponse::new().content("Something went wrong")).await {
                error!("Something went wrong {e:?}");
            }
            return Ok(())
        },
    };

    // Combine with the timezone
    let naive_date = NaiveDate::from_str(&date)
        .expect("Failed to parse date string");
    let datetime: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(naive_date.and_hms_opt(0, 0, 0).unwrap(), Utc);

    // Interact with the database
    let mut database = USER_DATABASE.lock().await;
    database.add_user(UserData {
        uuid: interaction.user.id.get(),
        likes_uwu: enjoyer,
        birthday: datetime,
    });

    database.save().await.unwrap();

    drop(database);

    if let Err(e) = interaction.edit_response(&ctx.http, EditInteractionResponse::new().content("Done!")).await {
        error!("Something went wrong {e:?}");
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("birthday")
        .description("Registers your birthday!")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "enjoyer",
                "Are you a enjoyer of uwu?",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "date",
                "The day you were born (yyyy-mm-dd)",
            )
            .required(true),
        )
}
