use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
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
    let mut tzone: Option<Tz> = None;
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
                "timezone" => {
                    if let Ok(parsed_tz) = Tz::from_str(value) {
                        tzone = Some(parsed_tz);
                    }
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
        None => return Ok(()),
    };
    
    let tzone = match tzone {
        Some(tz) => tz,
        None => return Ok(()),
    };

    let enjoyer = match enjoyer {
        Some(e) => e,
        None => return Ok(()),
    };

    // Parse the date string into a NaiveDateTime
    let naive = match NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S") {
        Ok(dt) => dt,
        Err(_) => return Ok(()),
    };

    // Combine with the timezone
    let datetime = match tzone.from_local_datetime(&naive).single() {
        Some(dt) => dt,
        None => return Ok(()),
    };

    // Convert to UTC
    let utc: DateTime<Utc> = datetime.with_timezone(&Utc);

    // Interact with the database
    let mut database = USER_DATABASE.lock().await;
    database.add_user(UserData {
        uuid: interaction.user.id.get(),
        likes_uwu: enjoyer,
        birthday: utc,
    });

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
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "timezone",
                "The timezone you live in. (eg: EST, CEST. Alt UTC works)",
            )
            .required(true),
        )
}
