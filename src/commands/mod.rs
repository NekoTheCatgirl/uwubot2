pub mod ban;
pub mod leaderboard;
pub mod ping;
pub mod unban;
pub mod uwu;

pub async fn register(ctx: &serenity::all::Context) {
    let _ = serenity::all::Command::create_global_command(&ctx.http, ban::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, unban::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, ping::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, leaderboard::register()).await;
    let _ = serenity::all::Command::create_global_command(&ctx.http, uwu::register()).await;
}
