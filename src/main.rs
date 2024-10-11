mod banned;
mod checks;
mod commands;
mod leaderboard;
mod logger;
mod matter;
mod immune;

use checks::trigger_check;
use immune::ImmuneUsers;
use lazy_static::lazy_static;
use log::{error, info, warn};
use logger::setup_logger;
use matter::{get_theme_based_on_date, MatterDict, MatterTrait};
use rand::rngs::OsRng;
use rand::Rng;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, Entitlement, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::banned::BannedChannels;
use crate::commands::register;
use crate::leaderboard::Leaderboard;

lazy_static! {
    static ref LEADERBOARD: Arc<Mutex<Leaderboard>> = Arc::new(Mutex::new(Leaderboard::load()));
    static ref BANNED: Arc<Mutex<BannedChannels>> = Arc::new(Mutex::new(BannedChannels::load()));
    static ref IMMUNE: Arc<Mutex<ImmuneUsers>> = Arc::new(Mutex::new(ImmuneUsers::load()));
}

const TOKEN: &str = include_str!("../token.tok");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        // Handle messages:
        message_fn(ctx, message).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Handle commands:
        interaction_create_fn(ctx, interaction).await;
    }

    async fn entitlement_create(&self, ctx: Context, entitlement: Entitlement) {
        entitlement_create_fn(ctx, entitlement).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // Handle registration:
        ready_fn(ctx, ready).await;
    }
}

async fn message_fn(ctx: Context, message: Message) {
    // Handle messages:
    if !message.author.bot && message.channel(&ctx.http).await.unwrap().guild().is_some()
    {
        let userid = message.author.id.get();
        let immune = {
            let immune = IMMUNE.lock().await;
            immune.users.contains(&userid)
        };
        if immune {
            return;
        }
        let matter = MatterDict::load().await.unwrap();
        let theme = get_theme_based_on_date().await;
        let true_matter = matter.get(theme).unwrap();
        let mut rng = OsRng;
        if trigger_check(&mut rng, true_matter, &message).await {
            if message.author.id == 248835673669369856 || message.author.id == 267245400363106304 {
                return;
            }
            ctx.http.start_typing(message.channel_id);
            let mut leaderboard = LEADERBOARD.lock().await;
            leaderboard.total_uwu += 1;
            let person = message.author.id.get();
            if let Some(old) = leaderboard.leaderboard.clone().get(&person) {
                leaderboard.leaderboard.insert(person, old + 1);
            } else {
                leaderboard.leaderboard.insert(person, 1);
            }
            let _ = leaderboard.save().await;

            let mut msg = String::new();

            let random = rng.gen_range(0..1000);
            if random <= 500 {
                // Original
                msg.push_str(&true_matter.gen_permutation(&mut rng));
                if let Err(why) = message.reply_ping(&ctx.http, msg).await {
                    error!("Error sending message: {why:?}");
                }
            } else {
                // Long form message
                msg = true_matter.get_long(&mut rng);
                msg = msg.replace("[User]", &message.author.name);
                if let Err(why) = message.reply_ping(&ctx.http, msg).await {
                    error!("Error sending message: {why:?}");
                }
            }
        }
    }
}

async fn interaction_create_fn(ctx: Context, interaction: Interaction) {
    // Handle commands:
    if let Interaction::Command(command) = interaction {
        info!("Recieved a command interaction: {command:#?}");

        let content = match command.data.name.as_str() {
            "ping" => Some(commands::ping::run()),
            "uwu" => Some(commands::uwu::run()),
            "ban" => {
                if let Err(err) = commands::ban::run(&ctx, &command).await {
                    error!("Failed to execute 'ban' command: {:?}", err);
                }
                None
            }
            "unban" => {
                if let Err(err) = commands::unban::run(&ctx, &command).await {
                    error!("Failed to execute 'unban' command: {:?}", err);
                }
                None
            }
            "leaderboard" => {
                if let Err(err) = commands::leaderboard::run(&ctx, &command).await {
                    error!("Failed to execute 'leaderboard' command: {:?}", err);
                }
                None
            }
            _ => Some("Not implemented yet >:<".to_string()),
        };

        if let Some(content) = content {
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                error!("Cannot respond to slash command: {why}");
            }
        }
    }
}

async fn entitlement_create_fn(ctx: Context, entitlement: Entitlement) {
    let skuid = entitlement.sku_id.get();
    match skuid {
        // Support the dev
        1284843842947907667 => {
            // Send out a thank you
            let user_id = entitlement.user_id.unwrap();
            let user = match ctx.http.get_user(user_id).await {
                Ok(user) => user,
                Err(why) => {
                    error!("Couldnt grab the user: {why}");
                    return;
                }
            };
            let channel = match user.create_dm_channel(&ctx.http).await {
                Ok(channel) => channel,
                Err(why) => {
                    error!("Couldnt create a private channel: {why}");
                    return;
                }
            };
            if let Err(why) = channel.say(&ctx.http, "Thank you so much for supporting the continued development and hosting of this bot!").await {
                error!("Couldnt send thank you to the user {user_id}, {why}");
            }
        }
        // Immunity permanent
        1284845359994110043 => {
            let user_id = entitlement.user_id.unwrap();
            // Make the user immune
            {
                let mut immune = IMMUNE.lock().await;
                immune.users.push(user_id.get());
                if let Err(why) = immune.save().await {
                    error!("Coudlnt save the immune file! {why}");
                }
            }
            // Send out a thank you
            let user = match ctx.http.get_user(user_id).await {
                Ok(user) => user,
                Err(why) => {
                    error!("Couldnt grab the user: {why}");
                    return;
                }
            };
            let channel = match user.create_dm_channel(&ctx.http).await {
                Ok(channel) => channel,
                Err(why) => {
                    error!("Couldnt create a private channel: {why}");
                    return;
                }
            };
            if let Err(why) = channel.say(&ctx.http, "Thank you for purchasing the immunity card. This is not reversable and you will now never be messaged randomly in guilds.").await {
                error!("Couldnt send thank you to the user {user_id}, {why}");
            }
        }
        _ => {
            // Invalid skuid
            warn!("Invalid skuid recieved");
        }
    }
}

async fn ready_fn(ctx: Context, ready: Ready) {
    // Handle registration:
    info!("{} is connected!", ready.user.name);

    register(&ctx).await;
}

#[tokio::main]
async fn main() {
    setup_logger(log::LevelFilter::Error);

    {
        let _ = BANNED.lock().await.save().await;
        let _ = LEADERBOARD.lock().await.save().await;
        let _ = IMMUNE.lock().await.save().await;
    }
        
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
