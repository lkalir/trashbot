//! Modify campaign levels

use super::{save_db, MySmolStr, LEVEL_MAP};
use log::warn;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use smol_str::SmolStr;

/// Set your levels
#[command]
#[min_args(1)]
#[max_args(2)]
#[only_in(guilds)]
#[usage = "campaign [level between 0 and 255: default increment]"]
pub async fn level(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut my_args = args.clone();
    let campaign: SmolStr = my_args.single_quoted::<String>()?.into();
    let level = *LEVEL_MAP
        .lock()
        .await
        .entry((msg.guild_id.unwrap(), MySmolStr(campaign.clone())))
        .and_modify(|l| *l = my_args.parse().unwrap_or(*l + 1))
        .or_insert_with(|| my_args.parse().unwrap_or(1));

    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, format!("'{}' is now level {}", campaign, level))
        .await
    {
        warn!("Failed to set campaign level {:?}", why);
    }

    save_db(&*LEVEL_MAP.lock().await).await
}
