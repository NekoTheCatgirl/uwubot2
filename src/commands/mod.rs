pub mod ban;
pub mod unban;
pub mod ping;
pub mod leaderboard;
pub mod uwu;
pub mod reg_birthday;

pub async fn register(ctx: &serenity::all::Context) {
    let _ = serenity::all::Command::create_global_command(&ctx.http, ban::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, unban::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, ping::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, leaderboard::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, uwu::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, reg_birthday::register()).await;
}