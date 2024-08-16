mod banned;
mod checks;
mod commands;
mod happy_birthday;
mod leaderboard;
mod long_form;
mod matter;
mod users;
mod logger;

use checks::trigger_check;
use lazy_static::lazy_static;
use log::{error, info};
use logger::setup_logger;
use long_form::get_options;
use matter::{get_theme_based_on_date, MatterDict, MatterTrait};
use rand::rngs::OsRng;
use rand::Rng;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::banned::BannedChannels;
use crate::commands::register;
use crate::happy_birthday::happy_birthday_loop;
use crate::leaderboard::Leaderboard;
use crate::users::UserDatabase;

lazy_static! {
    static ref LEADERBOARD: Arc<Mutex<Leaderboard>> = Arc::new(Mutex::new(Leaderboard::load()));
    static ref BANNED: Arc<Mutex<BannedChannels>> = Arc::new(Mutex::new(BannedChannels::load()));
    static ref USER_DATABASE: Arc<Mutex<UserDatabase>> = Arc::new(Mutex::new(UserDatabase::load()));
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

    async fn ready(&self, ctx: Context, ready: Ready) {
        // Handle registration:
        ready_fn(ctx, ready).await;
    }
}

async fn message_fn(ctx: Context, message: Message) {
    // Handle messages:
    if message.author.bot == false
        && message.channel(&ctx.http).await.unwrap().guild().is_none() == false
    {
        {
            let user_db = USER_DATABASE.lock().await;
            if let Some(user) = user_db.users.iter().find(|user| user.uuid == message.author.id.get()) {
                if user.likes_uwu == false {
                    return;
                }
            }
        }
        let matter = MatterDict::load().await.unwrap();
        let theme = get_theme_based_on_date(message.author.id.get()).await;
        let true_matter = matter.get(theme).unwrap();
        let mut rng = OsRng;
        if trigger_check(&mut rng, true_matter, &ctx, &message).await {
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
                let options = get_options().await.unwrap();
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
            "birthday" => {
                if let Err(err) = commands::reg_birthday::run(&ctx, &command).await {
                    error!("Failed to execute 'birthday' command: {:?}", err);
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

async fn ready_fn(ctx: Context, ready: Ready) {
    // Handle registration:
    info!("{} is connected!", ready.user.name);

    register(&ctx).await;

    tokio::spawn(happy_birthday_loop(ctx)).await.unwrap();
}

#[tokio::main]
async fn main() {
    setup_logger(log::LevelFilter::Debug);

    {
        let _ = BANNED.lock().await.save().await;
        let _ = LEADERBOARD.lock().await.save().await;
        let _ = USER_DATABASE.lock().await.save().await;
    }

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
