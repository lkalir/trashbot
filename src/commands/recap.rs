//! Posts the recap image

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

const RECAP_URL: &str = "https://i.imgur.com/LPKt2uF.png";

/// It's time for the recap
#[command]
#[description = "Time for the recap"]
pub async fn recap(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            format!("It's time for the r-r-r-r-recap!\n{}", RECAP_URL),
        )
        .await?;
    Ok(())
}
