use crate::{client::bot::StarboardBot, errors::StarboardResult};

pub async fn is_guild_premium(
    _bot: &StarboardBot,
    guild_id: i64,
    _allow_cache: bool,
) -> StarboardResult<bool> {
    let _ = guild_id;
    Ok(true)
}
