use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "UwU!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("uwu").description("A uwu command :D")
}