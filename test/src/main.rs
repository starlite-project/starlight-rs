use base64::decode;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let token = env::var("DISCORD_TOKEN")?;

    let split = token.split(".").collect::<Vec<_>>();

    let first = split.first().unwrap();

    let decoded = decode(first)?;

    dbg!(String::from_utf8(decoded)?);

    Ok(())
}
