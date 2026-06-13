use twilight_interactions::command::CommandModel;

use crate::{
    errors::StarboardResult,
    interactions::{commands::chat, context::CommandCtx},
};

const CONFIG_COMMANDS: &[&str] = &[
    "autostar",
    "starboards",
    "overrides",
    "exclusive-groups",
    "permroles",
    "filters",
    "xproles",
    "posroles",
    "utils",
];

macro_rules! match_commands {
    ($ctx:expr, $($cmd_name:expr => $command:ty),* $(,)?) => {
        let cmd_inp_data = $ctx.data.clone().into();
        match &*$ctx.data.name {
            $(
                $cmd_name => <$command>::from_interaction(cmd_inp_data)?.callback($ctx).await?,
            )*
            unknown => eprintln!("Unknown command: {}", unknown),
        }
    };
}

fn has_config_access(ctx: &CommandCtx) -> bool {
    let Some(member) = &ctx.interaction.member else {
        return false;
    };
    let Some(user) = &member.user else {
        return false;
    };

    ctx.bot.config.config_user_ids.contains(&user.id)
        || ctx.bot.config.owner_ids.contains(&user.id)
        || member
            .roles
            .iter()
            .any(|role_id| ctx.bot.config.config_role_ids.contains(role_id))
}

async fn require_config_access(ctx: &mut CommandCtx) -> StarboardResult<bool> {
    if has_config_access(ctx) {
        return Ok(true);
    }

    ctx.respond_str("You don't have permission to configure this bot.", true)
        .await?;

    Ok(false)
}

pub async fn handle_command(mut ctx: CommandCtx) -> StarboardResult<()> {
    if CONFIG_COMMANDS.contains(&&*ctx.data.name) && !require_config_access(&mut ctx).await? {
        return Ok(());
    }

    match_commands!(
        ctx,
        "ping" => chat::ping::Ping,
        "help" => chat::help::Help,
        "botstats" => chat::botstats::BotStats,
        "leaderboard" => chat::leaderboard::Leaderboard,
        "stats" => chat::stats::Stats,
        "random" => chat::random::RandomPost,
        "moststarred" => chat::moststarred::MostStarred,
        "autostar" => chat::autostar::AutoStar,
        "starboards" => chat::starboard::Starboard,
        "overrides" => chat::overrides::Overrides,
        "exclusive-groups" => chat::exclusive_groups::ExclusiveGroups,
        "permroles" => chat::permroles::PermRoles,
        "filters" => chat::filters::Filters,
        "xproles" => chat::xproles::XPRoles,
        "posroles" => chat::posroles::PosRoles,
        "utils" => chat::utils::Utils,
    );

    Ok(())
}
