//! Processing for actions based on passively reading regular, non-command messages

mod arch;
mod birbs;
mod gnu;

use arch::is_arch;
use birbs::{is_bird, BURD};
use gnu::{is_gnu, STALLMAN};
use log::error;
use serenity::{
    client::Context,
    framework::standard::macros::hook,
    model::{channel::Message, id::EmojiId, misc::EmojiIdentifier},
};
use std::iter::repeat;

/// Responses to normal messages i.e. not commands
///
/// This callback is ran against every non-command message in a channel visible to trashbot
///
/// TODO: Tweak the space count on the "woah" reaction. Too much? Too little? Also come up with
/// better snark
///
/// TODO: Abstract out the guild emojis, at least collect them into a common location as statics
#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    let scontent = msg.content.to_lowercase();
    if is_bird(&scontent) {
        let birds: String = repeat(":bird:").take(17).collect();
        let mes = format!("{}\n{}\n{}", birds, BURD, birds);
        if let Err(why) = msg.reply_ping(&ctx.http, mes).await {
            error!("Failed to bird up: {:?}", why);
        };
    } else if is_arch(&scontent) {
        let emoj = EmojiIdentifier {
            animated: false,
            id: EmojiId(727294348965707786),
            name: "arch_btw".to_string(),
        };

        if let Err(why) = msg.react(&ctx.http, emoj).await {
            error!("Failed to btw: {:?}", why);
        }
    } else if is_gnu(&scontent) {
        if let Err(why) = msg.reply_ping(&ctx.http, STALLMAN).await {
            error!("Failed to actually: {:?}", why);
        }
    } else if msg.content.matches(' ').count() > 40 {
        if let Err(why) = msg.reply_ping(&ctx.http, ":woah:... chill out").await {
            error!("Failed to woah: {:?}", why);
        }
    }
}
