use serenity::builder::CreateCommand;

pub fn run() -> String {
    "UwU!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("uwu").description("A uwu command :D")
}
