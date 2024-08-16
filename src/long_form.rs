use tokio::fs;

type LongFormOptions = Vec<String>;

pub async fn get_options() -> Result<LongFormOptions, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./messages.json").await?;
    let options: LongFormOptions = serde_json::from_str(&contents)?;
    Ok(options)
}
