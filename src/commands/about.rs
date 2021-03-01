//! Trash bot info (not help) command

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

/// Trash bot lore
///
/// TODO: GREATLY expand this
#[command]
#[description = "Let me tell you the tale of my people."]
pub async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "I am trash-bot! :^)").await?;
    Ok(())
}
