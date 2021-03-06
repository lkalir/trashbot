//! The actions the bot can take

mod about;
mod laprate;
mod levelup;
mod recap;
mod stalker;
mod wha_happun;

use about::*;
use laprate::*;
use levelup::{delete::*, level::*, wlaw::*};
use log::{error, info, warn};
use recap::*;
use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::{group, help, hook},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};
pub use stalker::normal_message;
use std::collections::HashSet;
use wha_happun::*;

#[help]
#[command_not_found_text = "Could not find: '{}'."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

/// Callback called before any command is processed
#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}' from '{}' in '{}'",
        command_name,
        msg.author.name,
        msg.guild(ctx)
            .await
            .map(|g| g.name)
            .unwrap_or_else(|| "???".to_string()),
        msg.channel_id
            .name(ctx)
            .await
            .unwrap_or_else(|| "???".to_string())
    );
    true
}

/// Callback called after processing a command
#[hook]
pub async fn after(
    _ctx: &Context,
    _msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
    }
}

/// Callback called when an invalid command is called
#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    warn!("Could not find command '{}'", unknown_command_name);
}

#[group]
#[commands(about, wha_happun, recap, laprate, wlaw, level, delete)]
pub struct General;
