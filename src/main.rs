//! Trash-bot is a garbage bot for a garbage server. It mostly just makes snarky comments, but one
//! day, I believe trash-bot will change the world.

mod commands;

use commands::{after, before, normal_message, unknown_command};
use log::{error, info};
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::StandardFramework,
    http::Http,
    model::gateway::Ready,
};
use std::{collections::HashSet, env};

/// ZST for EventHandler impl
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
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
                .prefix("~")
                .delimiters(vec![", ", ","])
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
        .await
        .map_err(|why| error!("Err creating client: {:?}", why))?;

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
