//! Trash bot info (not help) command

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

/// What is trashbot?
#[command]
pub async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "I am trash-bot! :^)").await?;
    Ok(())
}
