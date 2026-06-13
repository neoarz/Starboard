use crate::{client::bot::StarboardBot, errors::StarboardResult, utils::into_id::IntoId};

pub async fn refresh_premium_locks(
    bot: &StarboardBot,
    guild_id: i64,
    _premium: bool,
) -> StarboardResult<()> {
    sqlx::query!(
        "UPDATE starboards SET premium_locked=false WHERE guild_id=$1",
        guild_id
    )
    .fetch_all(&bot.pool)
    .await?;
    let unlocked_asc_channel_ids = sqlx::query!(
        "UPDATE autostar_channels SET premium_locked=false WHERE guild_id=$1
        RETURNING channel_id",
        guild_id
    )
    .fetch_all(&bot.pool)
    .await?;
    for row in unlocked_asc_channel_ids {
        bot.cache
            .autostar_channel_ids
            .insert(row.channel_id.into_id());
    }

    Ok(())
}
