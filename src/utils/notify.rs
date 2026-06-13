use twilight_model::{
    channel::message::component::{ActionRow, Button, ButtonStyle, Component},
    id::{Id, marker::UserMarker},
};

use crate::{client::bot::StarboardBot, errors::StarboardResult};

use super::dm;

pub async fn notify(
    bot: &StarboardBot,
    user_id: Id<UserMarker>,
    message: &str,
) -> StarboardResult<()> {
    let comp = Component::ActionRow(ActionRow {
        components: vec![Component::Button(Button {
            sku_id: None,
            label: Some("Dismiss".to_string()),
            url: None,
            style: ButtonStyle::Secondary,
            custom_id: Some("stateless::dismiss_notification".to_string()),
            disabled: false,
            emoji: None,
            id: None,
        })],
        id: None,
    });

    let _ = dm::dm(bot, user_id)
        .await?
        .content(message)
        .components(&[comp])
        .await;

    Ok(())
}
