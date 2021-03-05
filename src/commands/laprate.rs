//! Modify how frequently trashbot claps back

use log::warn;
use once_cell::sync::Lazy;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
};
use std::collections::HashMap;
use tokio::sync::Mutex;

/// How often to *lap *lap, tracked by channel
pub static LAP_RATE: Lazy<Mutex<HashMap<ChannelId, f64>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Modifies the *lap *lap frequency, tracked by channel
#[command]
#[description = "Display or modify the *lap *lap frequency"]
#[min_args(0)]
#[max_args(1)]
#[usage = "[lap frequency, between 0 and 1]"]
pub async fn laprate(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Ok(new_rate) = args.clone().single() {
        if new_rate > 1.0 {
            if let Err(why) = msg
                .reply(&ctx.http, "Hey, bub, 0 to 1 inclusive only, please")
                .await
            {
                warn!("Failed to warn fool: {:?}", why);
            }
        } else {
            LAP_RATE.lock().await.insert(msg.channel_id, new_rate);

            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("New rate is {}", new_rate))
                .await
            {
                warn!("Failed to set rate: {:?}", why);
            }
        }
    } else {
        let mut hm = LAP_RATE.lock().await;
        let rate = hm.entry(msg.channel_id).or_insert(0.1);

        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("Current rate is {}", rate))
            .await
        {
            warn!("Failed to get rate: {:?}", why);
        }
    }
    Ok(())
}
