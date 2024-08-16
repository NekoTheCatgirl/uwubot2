use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run() -> String {
    "Pong!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command :D")
}
