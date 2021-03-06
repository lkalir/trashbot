//! Delete campaign levels

use super::{save_db, MySmolStr, LEVEL_MAP};
use log::warn;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use smol_str::SmolStr;

/// Delete campaign
#[command]
#[num_args(1)]
#[only_in(guilds)]
#[usage = "campaign"]
pub async fn delete(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut my_args = args.clone();
    let campaign: SmolStr = my_args.single_quoted::<String>()?.into();
    if LEVEL_MAP
        .lock()
        .await
        .remove_entry(&(msg.guild_id.unwrap(), MySmolStr(campaign.clone())))
        .is_some()
    {
        save_db(&*LEVEL_MAP.lock().await).await?;
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("Deleted '{}'", campaign))
            .await
        {
            warn!("Failed to delete campaign {:?}", why);
        }
    } else if let Err(why) = msg
        .channel_id
        .say(&ctx.http, format!("No such campaign '{}'", campaign))
        .await
    {
        warn!("Failed to delete campaign {:?}", why);
    }

    Ok(())
}
