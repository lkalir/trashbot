//! Checks campaign level

use super::{MySmolStr, LEVEL_MAP};
use log::warn;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use smol_str::SmolStr;

/// "What level are we?"
#[command]
#[aliases(whatlevelarewe)]
#[min_args(0)]
#[max_args(1)]
#[description = "What level you should be in a campaign"]
#[only_in(guilds)]
#[usage = "[campaign: default prints all campaigns in server]"]
pub async fn wlaw(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut my_args = args.clone();
    if let Ok(campaign) = my_args.single_quoted::<String>().map(SmolStr::new) {
        if let Some(level) = LEVEL_MAP
            .lock()
            .await
            .get(&(msg.guild_id.unwrap(), MySmolStr(campaign.clone())))
        {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("'{}' is level {}", campaign, level))
                .await
            {
                warn!("Failed to report campaign level: {:?}", why);
            }
        } else if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("No such campaign '{}'", campaign))
            .await
        {
            warn!("Failed to report campaign level: {:?}", why);
        }
    } else {
        let id = msg.guild_id.unwrap();
        let camps: Vec<String> = LEVEL_MAP
            .lock()
            .await
            .iter()
            .filter_map(|((guild, campaign), level)| {
                if *guild == id {
                    Some((campaign, level))
                } else {
                    None
                }
            })
            .map(|(campaign, level)| format!("'{}' is level {}", campaign.0, level))
            .collect();
        let camps = camps.join("\n");

        if camps.is_empty() {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "No campaigns tracked in server")
                .await
            {
                warn!("Failed to dump campaigns: {:?}", why);
            }
        } else if let Err(why) = msg.channel_id.say(&ctx.http, camps).await {
            warn!("Failed to dump campaigns: {:?}", why);
        }
    }

    Ok(())
}
