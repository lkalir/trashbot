//! Trash-bot is a garbage bot for a garbage server. It mostly just makes snarky comments, but one
//! day, I believe trash-bot will change the world.

mod commands;
mod utils;

use commands::{
    after, before,
    levelup::{delete, level, wlaw},
    normal_message, unknown_command,
};
use log::{error, info};
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::StandardFramework,
    http::Http,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
};
use std::{collections::HashSet, env};

const RECAP_URL: &str = "https://i.imgur.com/LPKt2uF.png";

/// ZST for EventHandler impl
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "about" => "I am trash-bot! :^)".to_string(),
                "recap" => format!("It's time for the r-r-r-r-recap!\n{}", RECAP_URL),
                "wlaw" => match (command.guild_id, command.data.options.get(0)) {
                    (Some(id), Some(opts)) => {
                        if let Some(ApplicationCommandInteractionDataOptionValue::String(v)) =
                            &opts.resolved
                        {
                            wlaw::wlaw(id, Some(v.clone().into())).await
                        } else {
                            "Missing option?".to_string()
                        }
                    }
                    (Some(id), None) => wlaw::wlaw(id, None).await,
                    _ => "Command must be used in a server!".to_string(),
                },
                "level" => match (
                    command.guild_id,
                    command.data.options.get(0),
                    command.data.options.get(1),
                ) {
                    (Some(id), Some(camp), lvl) => {
                        if let Some(ApplicationCommandInteractionDataOptionValue::String(v)) =
                            &camp.resolved
                        {
                            let lvl = if let Some(v) = lvl {
                                if let Some(
                                    ApplicationCommandInteractionDataOptionValue::Integer(val),
                                ) = &v.resolved
                                {
                                    Some(*val as u8)
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                            level::level(id, lvl, v.clone().into()).await
                        } else {
                            "Missing option?".to_string()
                        }
                    }
                    _ => {
                        "Command must be used in a server and must specify a campaign!".to_string()
                    }
                },
                "delete" => match (command.guild_id, command.data.options.get(0)) {
                    (Some(id), Some(opts)) => {
                        if let Some(ApplicationCommandInteractionDataOptionValue::String(v)) =
                            &opts.resolved
                        {
                            delete::delete(id, v.clone().into()).await
                        } else {
                            "Missing option?".to_string()
                        }
                    }
                    (Some(id), None) => wlaw::wlaw(id, None).await,
                    _ => "Command must be used in a server!".to_string(),
                },
                _ => "not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Could not respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("about").description("What is trashbot?")
                })
                .create_application_command(|command| {
                    command.name("recap").description("It's time for the recap")
                })
                .create_application_command(|command| {
                    command
                        .name("wlaw")
                        .description("What level you should be in a campaign, leave blank to show all campaigns")
                        .create_option(|option| {
                            option
                                .name("campaign")
                                .description("The campaign to check")
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_application_command(|command| {
                    command.name("level").description("Set level in a campaign").create_option(|option| {
                        option.name("campaign").description("The campaign to set").kind(ApplicationCommandOptionType::String).required(true)
                    }).create_option(|option| {
                        option.name("level").description("What level are we. Leave blank to increment current level").kind(ApplicationCommandOptionType::Integer)
                    })
                })
                .create_application_command(|command| {
                    command.name("delete").description("Remove a campaign").create_option(|option| {
                        option.name("campaign").description("The campaign to remove").kind(ApplicationCommandOptionType::String).required(true)
                    })
                })
        })
        .await;

        info!("I have the following slash commands: {:#?}", commands);
    }
}

/// Main program loop
///
/// Starts up the bot and logger, authenticates, and listens. Basically if anything unexpected
/// happens the whole thing burns down.
#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    // Is there a nicer way to get a similar-ish pattern?
    let token = env::var("DISCORD_TOKEN")
        .map_err(|why| error!("Expected a token in the env: {:?}", why))?;

    let app_id = env::var("DISCORD_APPLICATION_ID")
        .map_err(|why| error!("Expected an application ID in the env: {:?}", why))?
        .parse()
        .map_err(|why| error!("Application ID is invalid: {:?}", why))?;

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => {
                    Err(why).map_err(|why| error!("Could not access the bot id: {:?}", why))?
                }
            }
        }
        Err(why) => {
            Err(why).map_err(|why| error!("Could not access application info: {:?}", why))?
        }
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("!")
                .delimiters(vec![" "])
                .owners(owners)
        })
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .help(&commands::MY_HELP)
        .group(&commands::GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(app_id)
        .await
        .map_err(|why| error!("Err creating client: {:?}", why))?;

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
