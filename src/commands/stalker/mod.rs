//! Processing for actions based on passively reading regular, non-command messages

mod arch;
mod birbs;
mod clapclap;
mod gnu;

use super::laprate::LAP_RATE;
use crate::utils::lookup_and_cache;
use arch::is_arch;
use birbs::{is_bird, BURD};
use clapclap::is_lap;
use gnu::{is_gnu, STALLMAN};
use log::error;
use rand::prelude::*;
use serenity::{client::Context, framework::standard::macros::hook, model::channel::Message};
use smol_str::SmolStr;
use std::iter::repeat;

/// Responses to normal messages i.e. not commands
///
/// This callback is ran against every non-command message in a channel visible to trashbot
///
/// TODO: Tweak the space count on the "woah" reaction. Too much? Too little? Also come up with
/// better snark
///
/// TODO: Is there a more elegant way to implement this pattern?
#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    let scontent = msg.content.to_lowercase();

    if is_bird(&scontent) {
        let birds: SmolStr = repeat(":bird:").take(17).collect();
        let mes = format!("{}\n{}\n{}", birds, BURD, birds);
        if let Err(why) = msg.reply_ping(&ctx.http, mes).await {
            error!("Failed to bird up: {:?}", why);
        };
    } else if is_arch(&scontent) {
        if let Some(emoj) =
            lookup_and_cache(msg.guild(ctx).await.unwrap().id, ctx, "arch_btw").await
        {
            if let Err(why) = msg.react(&ctx.http, emoj).await {
                error!("Failed to btw: {:?}", why);
            }
        }
    } else if is_gnu(&scontent) {
        if let Err(why) = msg.reply_ping(&ctx.http, STALLMAN).await {
            error!("Failed to actually: {:?}", why);
        }
    } else if let Some(lap) = is_lap(&scontent) {
        // Don't clap every time
        if *LAP_RATE.lock().await.entry(msg.channel_id).or_insert(0.1) > rand::thread_rng().gen() {
            if let Err(why) = msg
                .reply_ping(&ctx.http, format!("{} {} :clap: :clap:", lap, lap))
                .await
            {
                error!("Failed to clap: {:?}", why);
            }
        }
    } else if msg.content.matches(' ').count() > 40 {
        if let Some(woah) = lookup_and_cache(msg.guild(ctx).await.unwrap().id, ctx, "woah").await {
            if let Err(why) = msg
                .reply_ping(
                    &ctx.http,
                    format!("<:{}:{}>... chill out", woah.name, woah.id,),
                )
                .await
            {
                error!("Failed to woah: {:?}", why);
            }
        }
    }
}
